mod formats;

use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;
use globset::{Glob, GlobSet, GlobSetBuilder};

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

fn mk_globset(elems: Vec<String>) -> GlobSet {
    if elems.is_empty() {
        return GlobSet::empty();
    }

    let mut globset = GlobSetBuilder::new();
    elems.iter().filter(|e| !e.is_empty()).for_each(|e| {
        if let Ok(glob) = Glob::new(e) {
            globset.add(glob);
        }
    });
    globset.build().map_or(GlobSet::empty(), |globset| globset)
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

#[derive(Parser, Debug)]
#[clap(
    name = "rust-code-analysis-cli",
    version,
    author,
    about = "Analyze source code."
)]
struct Opts {
    /// Input files to analyze.
    #[clap(long, short, value_parser)]
    paths: Vec<PathBuf>,
    /// Output AST to stdout.
    #[clap(long, short)]
    dump: bool,
    /// Remove comments in the specified files.
    #[clap(long, short)]
    comments: bool,
    /// Find nodes of the given type.
    #[clap(long, short, number_of_values = 1)]
    find: Vec<String>,
    /// Get functions and their spans.
    #[clap(long, short = 'F')]
    function: bool,
    /// Count nodes of the given type: comma separated list.
    #[clap(long, short = 'C', number_of_values = 1)]
    count: Vec<String>,
    /// Compute different metrics.
    #[clap(long, short)]
    metrics: bool,
    /// Retrieve all operands and operators in a code.
    #[clap(long, conflicts_with = "metrics")]
    ops: bool,
    /// Do action in place.
    #[clap(long, short)]
    in_place: bool,
    /// Glob to include files.
    #[clap(long, short = 'I', num_args(0..))]
    include: Vec<String>,
    /// Glob to exclude files.
    #[clap(long, short = 'X', num_args(0..))]
    exclude: Vec<String>,
    /// Number of jobs.
    #[clap(long, short = 'j')]
    num_jobs: Option<usize>,
    /// Language type.
    #[clap(long, short)]
    language_type: Option<String>,
    /// Output metrics as different formats.
    #[clap(long, short = 'O', value_parser = PossibleValuesParser::new(Format::all())
        .map(|s| s.parse::<Format>().unwrap()))]
    output_format: Option<Format>,
    /// Dump a pretty json file.
    #[clap(long = "pr")]
    pretty: bool,
    /// Output file/directory.
    #[clap(long, short, value_parser)]
    output: Option<PathBuf>,
    /// Get preprocessor declaration for C/C++.
    #[clap(long, value_parser, number_of_values = 1)]
    preproc: Vec<PathBuf>,
    /// Line start.
    #[clap(long = "ls")]
    line_start: Option<usize>,
    /// Line end.
    #[clap(long = "le")]
    line_end: Option<usize>,
    /// Print the warnings.
    #[clap(long, short)]
    warning: bool,
}

fn main() {
    let opts = Opts::parse();

    let count_lock = if !opts.count.is_empty() {
        Some(Arc::new(Mutex::new(Count::default())))
    } else {
        None
    };

    let (preproc_lock, preproc) = match opts.preproc.len().cmp(&1) {
        Ordering::Equal => {
            let data = read_file(&opts.preproc[0]).unwrap();
            eprintln!("Load preproc data");
            let x = (
                None,
                Some(Arc::new(
                    serde_json::from_slice::<PreprocResults>(&data).unwrap(),
                )),
            );
            eprintln!("Load preproc data: finished");
            x
        }
        Ordering::Greater => (Some(Arc::new(Mutex::new(PreprocResults::default()))), None),
        Ordering::Less => (None, None),
    };

    let output_is_dir = opts.output.as_ref().map(|p| p.is_dir()).unwrap_or(false);
    if (opts.metrics || opts.ops) && opts.output.is_some() && !output_is_dir {
        eprintln!("Error: The output parameter must be a directory");
        process::exit(1);
    }

    let typ = opts.language_type.unwrap_or_default();
    let language = if preproc_lock.is_some() {
        Some(LANG::Preproc)
    } else if typ.is_empty() {
        None
    } else if typ == "ccomment" {
        Some(LANG::Ccomment)
    } else if typ == "preproc" {
        Some(LANG::Preproc)
    } else {
        get_from_ext(&typ)
    };

    let num_jobs = opts
        .num_jobs
        .map(|num_jobs| std::cmp::max(2, num_jobs) - 1)
        .unwrap_or_else(|| {
            std::cmp::max(
                2,
                available_parallelism()
                    .expect("Unrecoverable: Failed to get thread count")
                    .get(),
            ) - 1
        });

    let include = mk_globset(opts.include);
    let exclude = mk_globset(opts.exclude);

    let cfg = Config {
        dump: opts.dump,
        in_place: opts.in_place,
        comments: opts.comments,
        find_filter: opts.find,
        count_filter: opts.count,
        language,
        function: opts.function,
        metrics: opts.metrics,
        ops: opts.ops,
        output_format: opts.output_format,
        pretty: opts.pretty,
        output: opts.output.clone(),
        line_start: opts.line_start,
        line_end: opts.line_end,
        preproc_lock: preproc_lock.clone(),
        preproc,
        count_lock: count_lock.clone(),
    };

    let files_data = FilesData {
        include,
        exclude,
        paths: opts.paths,
    };

    let all_files = match ConcurrentRunner::new(num_jobs, act_on_file)
        .set_proc_dir_paths(process_dir_path)
        .run(cfg, files_data)
    {
        Ok(all_files) => all_files,
        Err(e) => {
            eprintln!("{e:?}");
            process::exit(1);
        }
    };

    if let Some(count) = count_lock {
        let count = Arc::try_unwrap(count).unwrap().into_inner().unwrap();
        println!("{count}");
    }

    if let Some(preproc) = preproc_lock {
        let mut data = Arc::try_unwrap(preproc).unwrap().into_inner().unwrap();
        fix_includes(&mut data.files, &all_files);

        let data = serde_json::to_string(&data).unwrap();
        if let Some(output_path) = opts.output {
            write_file(&output_path, data.as_bytes()).unwrap();
        } else {
            println!("{data}");
        }
    }
}
