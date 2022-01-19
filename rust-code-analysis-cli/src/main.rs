extern crate num_cpus;
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

mod formats;

use clap::{crate_version, App, Arg};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::{hash_map, HashMap};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use formats::Format;

// Enums
use rust_code_analysis::LANG;

// Structs
use rust_code_analysis::{
    CommentRm, CommentRmCfg, ConcurrentRunner, Count, CountCfg, Dump, DumpCfg, FilesData, Find,
    FindCfg, Function, FunctionCfg, Metrics, MetricsCfg, OpsCfg, OpsCode, PreprocParser,
    PreprocResults,
};

// Functions
use rust_code_analysis::{
    action, fix_includes, get_from_ext, get_function_spaces, get_ops, guess_language, preprocess,
    read_file, read_file_with_eol, write_file,
};

// Traits
use rust_code_analysis::ParserTrait;

#[derive(Debug)]
struct Config {
    dump: bool,
    in_place: bool,
    comments: bool,
    find_filter: Vec<String>,
    count_filter: Vec<String>,
    language: Option<LANG>,
    function: bool,
    metrics: bool,
    ops: bool,
    output_format: Option<Format>,
    output: Option<PathBuf>,
    pretty: bool,
    line_start: Option<usize>,
    line_end: Option<usize>,
    preproc_lock: Option<Arc<Mutex<PreprocResults>>>,
    preproc: Option<Arc<PreprocResults>>,
    count_lock: Option<Arc<Mutex<Count>>>,
}

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

fn act_on_file(path: PathBuf, cfg: &Config) -> std::io::Result<()> {
    let source = if let Some(source) = read_file_with_eol(&path)? {
        source
    } else {
        return Ok(());
    };

    let language = if let Some(language) = cfg.language {
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
            if let Some(space) = get_function_spaces(&language, source, &path, pr) {
                output_format.dump_formats(&space, &path, &cfg.output, cfg.pretty)
            } else {
                Ok(())
            }
        } else {
            let cfg = MetricsCfg { path };
            let path = cfg.path.clone();
            action::<Metrics>(&language, source, &path, pr, cfg)
        }
    } else if cfg.ops {
        if let Some(output_format) = &cfg.output_format {
            let ops = get_ops(&language, source, &path, pr).unwrap();
            output_format.dump_formats(&ops, &path, &cfg.output, cfg.pretty)
        } else {
            let cfg = OpsCfg { path };
            let path = cfg.path.clone();
            action::<OpsCode>(&language, source, &path, pr, cfg)
        }
    } else if cfg.comments {
        let cfg = CommentRmCfg {
            in_place: cfg.in_place,
            path,
        };
        let path = cfg.path.clone();
        if language == LANG::Cpp {
            action::<CommentRm>(&LANG::Ccomment, source, &path, pr, cfg)
        } else {
            action::<CommentRm>(&language, source, &path, pr, cfg)
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

fn process_dir_path(all_files: &mut HashMap<String, Vec<PathBuf>>, path: &Path, cfg: &Config) {
    if cfg.preproc_lock.is_some() {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        match all_files.entry(file_name) {
            hash_map::Entry::Occupied(l) => {
                l.into_mut().push(path.to_path_buf());
            }
            hash_map::Entry::Vacant(p) => {
                p.insert(vec![path.to_path_buf()]);
            }
        };
    }
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
    let matches = App::new("rust-code-analysis-cli")
        .version(crate_version!())
        .author(&*env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
        .about("Analyze source code")
        .arg(
            Arg::new("paths")
                .help("Sets the input files to analyze")
                .short('p')
                .long("paths")
                .default_value(".")
                .multiple_values(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("dump")
                .help("Outputs the AST to stdout")
                .short('d')
                .long("dump"),
        )
        .arg(
            Arg::new("remove_comments")
                .help("Remove comment in the specified files")
                .short('c')
                .long("comments"),
        )
        .arg(
            Arg::new("find")
                .help("Find nodes of the given type: comma separated list")
                .short('f')
                .long("find")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("function")
                .help("Get functions and their spans")
                .short('F')
                .long("function"),
        )
        .arg(
            Arg::new("count")
                .help("Count nodes of the given type: comma separated list")
                .short('C')
                .long("count")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("metrics")
                .help("Compute different metrics")
                .long("metrics")
                .short('m'),
        )
        .arg(
            Arg::new("ops")
                .help("Retrieves all operands and operators in a code")
                .long("ops")
                .conflicts_with("metrics"),
        )
        .arg(Arg::new("in_place").help("Do action in place").short('i'))
        .arg(
            Arg::new("include")
                .help("Glob to include files")
                .short('I')
                .long("include")
                .default_value("")
                .multiple_values(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("exclude")
                .help("Glob to exclude files")
                .short('X')
                .long("exclude")
                .default_value("")
                .multiple_values(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("num_jobs")
                .help("Number of jobs")
                .short('j')
                .value_name("NUMBER")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("language_type")
                .help("Language type")
                .short('l')
                .long("language-type")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("output_format")
                .help("Output metrics as different formats")
                .short('O')
                .long("output-format")
                .possible_values(Format::all())
                .takes_value(true),
        )
        .arg(
            Arg::new("pretty")
                .help("Dump a pretty json file")
                .long("pr"),
        )
        .arg(
            Arg::new("output")
                .help("Output file/directory")
                .short('o')
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::new("preproc")
                .help("Get preprocessor declaration for C/C++")
                .long("preproc")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("line_start")
                .help("Line start")
                .long("ls")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("line_end")
                .help("Line end")
                .long("le")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("warning")
                .help("Print the warnings")
                .long("warning")
                .short('w'),
        )
        .get_matches();

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
    let ops = matches.is_present("ops");
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
    let output = matches.value_of("output").map(PathBuf::from);
    let output_is_dir = output.as_ref().map(|p| p.is_dir()).unwrap_or(false);
    if (metrics || ops) && output.is_some() && !output_is_dir {
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

    let num_jobs = if let Ok(num_jobs) = matches.value_of("num_jobs").unwrap().parse::<usize>() {
        std::cmp::max(2, num_jobs) - 1
    } else {
        std::cmp::max(2, num_cpus::get()) - 1
    };

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

    let include = mk_globset(matches.values_of("include").unwrap());
    let exclude = mk_globset(matches.values_of("exclude").unwrap());

    let cfg = Config {
        dump,
        in_place,
        comments,
        find_filter,
        count_filter,
        language,
        function,
        metrics,
        ops,
        output_format,
        pretty,
        output: output.clone(),
        line_start,
        line_end,
        preproc_lock: preproc_lock.clone(),
        preproc,
        count_lock: count_lock.clone(),
    };

    let files_data = FilesData {
        include,
        exclude,
        paths,
    };

    let all_files = match ConcurrentRunner::new(num_jobs, act_on_file)
        .set_proc_dir_paths(process_dir_path)
        .run(cfg, files_data)
    {
        Ok(all_files) => all_files,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };

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
