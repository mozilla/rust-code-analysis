use globset::GlobSet;
use globset::{Glob, GlobSetBuilder};
use once_cell::sync::Lazy;
use rust_code_analysis::LANG;
use rust_code_analysis::*;
use std::path::Path;
use std::path::PathBuf;
use std::process;

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

    let mut settings = insta::Settings::new();

    settings.set_snapshot_path(
        Path::new("./repositories/rca-output/snapshots").join(path.strip_prefix(*REPO).unwrap()),
    );
    settings.set_snapshot_suffix(path.to_string_lossy());
    settings.bind(|| {
        insta::assert_debug_snapshot!(funcspace_struct);
    });

    Ok(())
}

static REPO: Lazy<&Path> = Lazy::new(|| Path::new("./tests/repositories"));

/// Produces metrics runtime and compares them with previously generated json files
fn compare_rca_output_with_files(repo_name: &str, include: &[&str]) {
    let num_jobs = 4;

    let cfg = Config { language: None };

    let mut gsbi = GlobSetBuilder::new();
    for file in include {
        gsbi.add(Glob::new(file).unwrap());
    }

    let files_data = FilesData {
        include: gsbi.build().unwrap(),
        exclude: GlobSet::empty(),
        paths: vec![REPO.join(repo_name)],
    };

    if let Err(e) = ConcurrentRunner::new(num_jobs, act_on_file).run(cfg, files_data) {
        eprintln!("{:?}", e);
        process::exit(1);
    }
}

#[test]
fn test_deepspeech() {
    compare_rca_output_with_files("DeepSpeech", &["*.cc", "*.cpp", "*.h", "*.hh"]);
}

#[test]
fn test_pdfjs() {
    compare_rca_output_with_files("pdf.js", &["*.js"]);
}

#[test]
fn test_serde() {
    compare_rca_output_with_files("serde", &["*.rs"]);
}
