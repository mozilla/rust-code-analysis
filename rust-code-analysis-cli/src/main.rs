#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate num_cpus;
#[macro_use]
extern crate serde;
extern crate serde_cbor;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

mod formats;
mod web;

use clap::{App, Arg};
use crossbeam::channel::{Receiver, Sender};
use crossbeam::crossbeam_channel::unbounded;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::{hash_map, HashMap};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{process, thread};
use walkdir::{DirEntry, WalkDir};

use formats::Format;
use rust_code_analysis::*;
use web::server;

#[derive(Debug)]
struct Config {
    dump: bool,
    in_place: bool,
    comments: bool,
    find_filter: Vec<String>,
    count_filter: Vec<String>,
    function: bool,
    metrics: bool,
    output_format: Option<Format>,
    output: Option<PathBuf>,
    pretty: bool,
    line_start: Option<usize>,
    line_end: Option<usize>,
    preproc_lock: Option<Arc<Mutex<PreprocResults>>>,
    preproc: Option<Arc<PreprocResults>>,
    count_lock: Option<Arc<Mutex<Count>>>,
}

struct JobItem {
    language: Option<LANG>,
    path: PathBuf,
    cfg: Arc<Config>,
}

type JobReceiver = Receiver<Option<JobItem>>;
type JobSender = Sender<Option<JobItem>>;

fn mk_globset(elems: clap::Values) -> GlobSet {
    let mut globset = GlobSetBuilder::new();
    for e in elems {
        if !e.is_empty() {
            if let Ok(glob) = Glob::new(e) {
                globset.add(glob);
            }
        }
    }
    if let Ok(globset) = globset.build() {
        globset
    } else {
        GlobSet::empty()
    }
}

fn act_on_file(language: Option<LANG>, path: PathBuf, cfg: &Config) -> std::io::Result<()> {
    let source = if let Some(source) = read_file_with_eol(&path)? {
        source
    } else {
        return Ok(());
    };

    let language = if let Some(language) = language {
        language
    } else if let Some(language) = guess_language(&source, &path).0 {
        language
    } else {
        return Ok(());
    };

    let pr = cfg.preproc.clone();
    if cfg.dump {
        let cfg = DumpCfg {
            line_start: cfg.line_start,
            line_end: cfg.line_end,
        };
        action::<Dump>(&language, source, &path, pr, cfg)
    } else if cfg.metrics {
        if let Some(output_format) = &cfg.output_format {
            let space = get_function_spaces(&language, source, &path, pr).unwrap();
            output_format.dump_formats(&space, &path, &cfg.output, cfg.pretty)
        } else {
            let cfg = MetricsCfg { path };
            action::<Metrics>(&language, source, &cfg.path.clone(), pr, cfg)
        }
    } else if cfg.comments {
        let cfg = CommentRmCfg {
            in_place: cfg.in_place,
            path,
        };
        if language == LANG::Cpp {
            action::<CommentRm>(&LANG::Ccomment, source, &cfg.path.clone(), pr, cfg)
        } else {
            action::<CommentRm>(&language, source, &cfg.path.clone(), pr, cfg)
        }
    } else if cfg.function {
        let cfg = FunctionCfg { path: path.clone() };
        action::<Function>(&language, source, &path, pr, cfg)
    } else if !cfg.find_filter.is_empty() {
        let cfg = FindCfg {
            path: path.clone(),
            filters: cfg.find_filter.clone(),
            line_start: cfg.line_start,
            line_end: cfg.line_end,
        };
        action::<Find>(&language, source, &path, pr, cfg)
    } else if cfg.count_lock.is_some() {
        let cfg = CountCfg {
            filters: cfg.count_filter.clone(),
            stats: cfg.count_lock.as_ref().unwrap().clone(),
        };
        action::<Count>(&language, source, &path, pr, cfg)
    } else if cfg.preproc_lock.is_some() {
        if let Some(language) = guess_language(&source, &path).0 {
            if language == LANG::Cpp {
                let mut results = cfg.preproc_lock.as_ref().unwrap().lock().unwrap();
                preprocess(
                    &PreprocParser::new(source, &path, None),
                    &path,
                    &mut results,
                );
            }
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn consumer(receiver: JobReceiver) {
    while let Ok(job) = receiver.recv() {
        if job.is_none() {
            break;
        }
        let job = job.unwrap();
        let path = job.path.clone();

        if let Err(err) = act_on_file(job.language, job.path, &job.cfg) {
            eprintln!("{:?} for file {:?}", err, path);
        }
    }
}

fn send_file(path: PathBuf, cfg: &Arc<Config>, language: Option<LANG>, sender: &JobSender) {
    sender
        .send(Some(JobItem {
            language,
            path,
            cfg: Arc::clone(cfg),
        }))
        .unwrap();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn explore(
    mut paths: Vec<String>,
    cfg: &Arc<Config>,
    include: GlobSet,
    exclude: GlobSet,
    language: Option<LANG>,
    sender: &JobSender,
) -> HashMap<String, Vec<PathBuf>> {
    let mut all_files: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in paths.drain(..) {
        let path = PathBuf::from(path);
        if !path.exists() {
            eprintln!("Warning: File doesn't exist: {}", path.to_str().unwrap());
            continue;
        }
        if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| !is_hidden(e))
            {
                let entry = entry.unwrap();
                let path = entry.path().to_path_buf();
                if (include.is_empty() || include.is_match(&path))
                    && (exclude.is_empty() || !exclude.is_match(&path))
                    && path.is_file()
                {
                    if cfg.preproc_lock.is_some() {
                        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        let path = path.clone();
                        match all_files.entry(file_name) {
                            hash_map::Entry::Occupied(l) => {
                                l.into_mut().push(path);
                            }
                            hash_map::Entry::Vacant(p) => {
                                p.insert(vec![path]);
                            }
                        };
                    }

                    send_file(path, &cfg, language, &sender);
                }
            }
        } else if (include.is_empty() || include.is_match(&path))
            && (exclude.is_empty() || !exclude.is_match(&path))
            && path.is_file()
        {
            send_file(path, &cfg, language, &sender);
        }
    }

    all_files
}

fn parse_or_exit<T>(s: &str) -> T
where
    T: FromStr,
    T::Err: fmt::Display,
{
    T::from_str(s).unwrap_or_else(|e| {
        eprintln!("Error:\n{}", e);
        process::exit(1);
    })
}

fn main() {
    let matches = App::new("code-analysis")
        .version(crate_version!())
        .author(&*env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
        .about("Analyze source code")
        .arg(
            Arg::with_name("paths")
                .help("Sets the input files to analyze")
                .short("p")
                .long("paths")
                .default_value(".")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dump")
                .help("Specifies the output file")
                .short("d")
                .long("dump"),
        )
        .arg(
            Arg::with_name("remove_comments")
                .help("Remove comment in the specified files")
                .short("c")
                .long("comments"),
        )
        .arg(
            Arg::with_name("find")
                .help("Find nodes of the given type: comma separated list")
                .short("f")
                .long("find")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("function")
                .help("Get functions and their spans")
                .short("F")
                .long("function"),
        )
        .arg(
            Arg::with_name("count")
                .help("Count nodes of the given type: comma separated list")
                .short("C")
                .long("count")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("metrics")
                .help("Compute different metrics")
                .long("metrics")
                .short("m"),
        )
        .arg(
            Arg::with_name("in_place")
                .help("Do action in place")
                .short("i"),
        )
        .arg(
            Arg::with_name("include")
                .help("Glob to include files")
                .short("I")
                .long("include")
                .default_value("")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude")
                .help("Glob to exclude files")
                .short("X")
                .long("exclude")
                .default_value("")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_jobs")
                .help("Number of jobs")
                .short("j")
                .value_name("NUMBER")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("language_type")
                .help("Language type")
                .short("l")
                .long("language-type")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_format")
                .help("Output metrics as different formats")
                .short("O")
                .long("output-format")
                .possible_values(Format::all())
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pretty")
                .help("Dump a pretty json file")
                .long("pr"),
        )
        .arg(
            Arg::with_name("output")
                .help("Output file/directory")
                .short("o")
                .long("output")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("preproc")
                .help("Get preprocessor declaration for C/C++")
                .long("preproc")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("line_start")
                .help("Line start")
                .long("ls")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("line_end")
                .help("Line end")
                .long("le")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("serve")
                .help("Run a web server")
                .long("serve"),
        )
        .arg(
            Arg::with_name("host")
                .help("Host for the web server")
                .long("host")
                .default_value("127.0.0.1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .help("Port for the web server")
                .long("port")
                .default_value("8080")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("warning")
                .help("Print the warnings")
                .long("warning")
                .short("w"),
        )
        .get_matches();

    let num_jobs = if let Ok(num_jobs) = matches.value_of("num_jobs").unwrap().parse::<usize>() {
        num_jobs
    } else {
        num_cpus::get()
    };

    let serve = matches.is_present("serve");
    if serve {
        let host = matches.value_of("host").unwrap();
        let port = if let Ok(port) = matches.value_of("port").unwrap().parse::<u16>() {
            port
        } else {
            eprintln!("Invalid port number");
            return;
        };
        if let Err(e) = server::run(host.to_string(), port, num_jobs) {
            eprintln!("Cannot run the server at {}:{}: {}", host, port, e);
        }
        return;
    }

    let paths: Vec<_> = matches.values_of("paths").unwrap().collect();
    let paths: Vec<String> = paths.iter().map(|x| (*x).to_string()).collect();
    let dump = matches.is_present("dump");
    let function = matches.is_present("function");
    let in_place = matches.is_present("in_place");
    let comments = matches.is_present("remove_comments");
    let find = matches.value_of("find").unwrap();
    let find_filter: Vec<_> = find
        .split(|c| c == ',')
        .filter(|k| !k.is_empty())
        .map(|s| s.to_string())
        .collect();
    let count = matches.value_of("count").unwrap();
    let count_filter: Vec<_> = count
        .split(|c| c == ',')
        .filter(|k| !k.is_empty())
        .map(|s| s.to_string())
        .collect();
    let count_lock = if matches.occurrences_of("count") != 0 {
        Some(Arc::new(Mutex::new(Count::default())))
    } else {
        None
    };
    let metrics = matches.is_present("metrics");
    let typ = matches.value_of("language_type").unwrap();
    let preproc_value = matches.value_of("preproc").unwrap();
    let (preproc_lock, preproc) = if !preproc_value.is_empty() {
        let path = PathBuf::from(preproc_value);
        let data = read_file(&path).unwrap();
        eprintln!("Load preproc data");
        let x = (
            None,
            Some(Arc::new(
                serde_json::from_slice::<PreprocResults>(&data).unwrap(),
            )),
        );
        eprintln!("Load preproc data: finished");
        x
    } else if matches.occurrences_of("preproc") != 0 {
        (Some(Arc::new(Mutex::new(PreprocResults::default()))), None)
    } else {
        (None, None)
    };

    let output_format = matches
        .value_of("output_format")
        .map(parse_or_exit::<Format>);
    let pretty = matches.is_present("pretty");
    let output = matches.value_of("output").map(|s| PathBuf::from(s));
    let output_is_dir = output.as_ref().map(|p| p.is_dir()).unwrap_or(false);
    if metrics && output.is_some() && !output_is_dir {
        eprintln!("Error: The output parameter must be a directory");
        process::exit(1);
    }
    let language = if preproc_lock.is_some() {
        Some(LANG::Preproc)
    } else if typ.is_empty() {
        None
    } else if typ == "ccomment" {
        Some(LANG::Ccomment)
    } else if typ == "preproc" {
        Some(LANG::Preproc)
    } else {
        get_from_ext(typ)
    };
    let num_jobs = std::cmp::max(2, num_jobs) - 1;

    let line_start = if let Ok(n) = matches.value_of("line_start").unwrap().parse::<usize>() {
        Some(n)
    } else {
        None
    };
    let line_end = if let Ok(n) = matches.value_of("line_end").unwrap().parse::<usize>() {
        Some(n)
    } else {
        None
    };

    let cfg = Arc::new(Config {
        dump,
        in_place,
        comments,
        find_filter,
        count_filter,
        function,
        metrics,
        output_format,
        pretty,
        output: output.clone(),
        line_start,
        line_end,
        preproc_lock: preproc_lock.clone(),
        preproc,
        count_lock: count_lock.clone(),
    });

    let (sender, receiver) = unbounded();

    let producer = {
        let sender = sender.clone();
        let include = mk_globset(matches.values_of("include").unwrap());
        let exclude = mk_globset(matches.values_of("exclude").unwrap());

        thread::Builder::new()
            .name(String::from("Producer"))
            .spawn(move || explore(paths, &cfg, include, exclude, language, &sender))
            .unwrap()
    };

    let mut receivers = Vec::with_capacity(num_jobs);
    for i in 0..num_jobs {
        let receiver = receiver.clone();

        let t = thread::Builder::new()
            .name(format!("Consumer {}", i))
            .spawn(move || {
                consumer(receiver);
            })
            .unwrap();

        receivers.push(t);
    }

    let all_files = if let Ok(res) = producer.join() {
        res
    } else {
        process::exit(1);
    };

    // Poison the receiver, now that the producer is finished.
    for _ in 0..num_jobs {
        sender.send(None).unwrap();
    }

    for receiver in receivers {
        if receiver.join().is_err() {
            process::exit(1);
        }
    }

    if let Some(count) = count_lock {
        let count = Arc::try_unwrap(count).unwrap().into_inner().unwrap();
        println!("{}", count);
    }

    if let Some(preproc) = preproc_lock {
        let mut data = Arc::try_unwrap(preproc).unwrap().into_inner().unwrap();
        fix_includes(&mut data.files, &all_files);

        let data = serde_json::to_string(&data).unwrap();
        if let Some(output_path) = output {
            write_file(&output_path, data.as_bytes()).unwrap();
        } else {
            println!("{}", data);
        }
    }
}

// cargo run --release -- -p ../mozilla-central.hg/ -j24 -X "**/third_party/rust/**/*.*" -X "**/*.mako.rs" -X "**/js/src/jit/**/*MacroAssembler*" -X "**/js/src/vm/Opcodes.h" -X "**/obj-x86_64-pc-linux-gnu/**/*.*" -i -c
