use std::path::Path;
use std::path::PathBuf;
use std::process;

use globset::GlobSet;
use globset::{Glob, GlobSetBuilder};

use rust_code_analysis::LANG;
use rust_code_analysis::*;

const REPO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/", "repositories");
const SNAPSHOT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/",
    "repositories/rca-output/snapshots"
);

#[derive(Debug)]
struct Config {
    language: Option<LANG>,
}

fn act_on_file(path: PathBuf, cfg: &Config) -> std::io::Result<()> {
    // Open file
    let source = if let Some(source) = read_file_with_eol(&path)? {
        source
    } else {
        return Ok(());
    };

    // Guess programming language
    let language = if let Some(language) = cfg.language {
        language
    } else if let Some(language) = guess_language(&source, &path).0 {
        language
    } else {
        return Ok(());
    };

    // Get FuncSpace struct
    let funcspace_struct = get_function_spaces(&language, source, &path, None).unwrap();

    insta::with_settings!({snapshot_path => Path::new(SNAPSHOT_PATH)
                .join(path.strip_prefix(Path::new(REPO)).unwrap())
                .parent()
                .unwrap(),
                prepend_module_to_snapshot => false,
                sort_maps => true,
    }, {
        insta::assert_yaml_snapshot!(
            path.file_name().unwrap().to_string_lossy().as_ref(),
            funcspace_struct,
            {
                // Round floating point values to three decimal places since the can differ from
                // system to system.
                ".spaces[].**.metrics.*.*" => insta::rounded_redaction(3),
                ".metrics.*.*" => insta::rounded_redaction(3),
                // Redact away the name since paths are different on different systems.
                ".name" => "[filepath]",
            }
        );

    });

    Ok(())
}

/// Produces metrics runtime and compares them with previously generated json files
pub fn compare_rca_output_with_files(repo_name: &str, include: &[&str]) {
    let num_jobs = 4;

    let cfg = Config { language: None };

    let mut gsbi = GlobSetBuilder::new();
    for file in include {
        gsbi.add(Glob::new(file).unwrap());
    }

    let files_data = FilesData {
        include: gsbi.build().unwrap(),
        exclude: GlobSet::empty(),
        paths: vec![Path::new(REPO).join(repo_name)],
    };

    if let Err(e) = ConcurrentRunner::new(num_jobs, act_on_file).run(cfg, files_data) {
        eprintln!("{e:?}");
        process::exit(1);
    }
}
