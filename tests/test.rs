use globset::GlobSet;
use globset::{Glob, GlobSetBuilder};
use once_cell::sync::Lazy;
use rust_code_analysis::LANG;
use rust_code_analysis::*;
use std::fs::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
struct Config {
    language: Option<LANG>,
    output_folder: PathBuf,
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

    // Build json file path
    let mut file_path = cfg.output_folder.join(path.strip_prefix("./").unwrap());
    file_path.set_extension("json");

    // Produce and compare metrics only if json file exists
    if file_path.exists() {
        // Get FuncSpace struct
        let funcspace_struct = get_function_spaces(&language, source, &path, None).unwrap();

        // Get Value struct from json file
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let value_struct = serde_json::from_reader(reader)?;

        // Compare the 2 structs
        compare_structs(&funcspace_struct, &value_struct);
    }

    Ok(())
}

/// Compares a FuncSpace and a Value field by field
fn compare_structs(funcspace: &FuncSpace, value: &serde_json::Value) {
    let fsname = funcspace
        .name
        .as_deref()
        .unwrap_or_default()
        .replace('\r', "");
    let name1 = Path::new(&fsname);
    let name2 = Path::new(value["name"].as_str().unwrap_or_default());
    assert_eq!(name1, name2);

    let start_line1 = funcspace.start_line;
    let start_line2 = value["start_line"].as_u64().unwrap() as usize;
    assert_eq!(start_line1, start_line2);

    let end_line1 = funcspace.end_line;
    let end_line2 = value["end_line"].as_u64().unwrap() as usize;
    assert_eq!(end_line1, end_line2);

    let kind1 = &format!("{}", funcspace.kind);
    let kind2 = value["kind"].as_str().unwrap_or_default();
    assert_eq!(kind1, kind2);

    let metrics1 = &funcspace.metrics;
    let metrics2 = &value["metrics"];

    let nargs1 = &metrics1.nargs;
    let nargs2 = &metrics2["nargs"];
    compare_f64(nargs1.fn_args_sum(), &nargs2["total_functions"]);
    compare_f64(nargs1.closure_args_sum(), &nargs2["total_closures"]);
    compare_f64(nargs1.fn_args_average(), &nargs2["average_functions"]);
    compare_f64(nargs1.closure_args_average(), &nargs2["average_closures"]);
    compare_f64(nargs1.nargs_total(), &nargs2["total"]);
    compare_f64(nargs1.nargs_average(), &nargs2["average"]);
    compare_f64(nargs1.fn_args_min(), &nargs2["functions_min"]);
    compare_f64(nargs1.fn_args_max(), &nargs2["functions_max"]);
    compare_f64(nargs1.closure_args_min(), &nargs2["closures_min"]);
    compare_f64(nargs1.closure_args_max(), &nargs2["closures_max"]);

    let nexits1 = &metrics1.nexits;
    let nexits2 = &metrics2["nexits"];
    compare_f64(nexits1.exit(), &nexits2["sum"]);
    compare_f64(nexits1.exit_average(), &nexits2["average"]);
    compare_f64(nexits1.exit(), &nexits2["min"]);
    compare_f64(nexits1.exit_average(), &nexits2["max"]);

    let cognitive1 = &metrics1.cognitive;
    let cognitive2 = &metrics2["cognitive"];
    compare_f64(cognitive1.cognitive_sum(), &cognitive2["sum"]);
    compare_f64(cognitive1.cognitive_average(), &cognitive2["average"]);
    compare_f64(cognitive1.cognitive_min(), &cognitive2["min"]);
    compare_f64(cognitive1.cognitive_max(), &cognitive2["max"]);

    let cyclomatic1 = &metrics1.cyclomatic;
    let cyclomatic2 = &metrics2["cyclomatic"];
    compare_f64(cyclomatic1.cyclomatic_sum(), &cyclomatic2["sum"]);
    compare_f64(cyclomatic1.cyclomatic_average(), &cyclomatic2["average"]);
    compare_f64(cyclomatic1.cyclomatic_min(), &cyclomatic2["min"]);
    compare_f64(cyclomatic1.cyclomatic_max(), &cyclomatic2["max"]);

    let halstead1 = &metrics1.halstead;
    let halstead2 = &metrics2["halstead"];
    compare_f64(halstead1.u_operators(), &halstead2["n1"]);
    compare_f64(halstead1.operators(), &halstead2["N1"]);
    compare_f64(halstead1.u_operands(), &halstead2["n2"]);
    compare_f64(halstead1.operands(), &halstead2["N2"]);
    compare_f64(halstead1.length(), &halstead2["length"]);
    compare_f64(
        halstead1.estimated_program_length(),
        &halstead2["estimated_program_length"],
    );
    compare_f64(halstead1.purity_ratio(), &halstead2["purity_ratio"]);
    compare_f64(halstead1.vocabulary(), &halstead2["vocabulary"]);
    compare_f64(halstead1.volume(), &halstead2["volume"]);
    compare_f64(halstead1.difficulty(), &halstead2["difficulty"]);
    compare_f64(halstead1.level(), &halstead2["level"]);
    compare_f64(halstead1.effort(), &halstead2["effort"]);
    compare_f64(halstead1.time(), &halstead2["time"]);
    compare_f64(halstead1.bugs(), &halstead2["bugs"]);

    let loc1 = &metrics1.loc;
    let loc2 = &metrics2["loc"];
    compare_f64(loc1.sloc(), &loc2["sloc"]);
    compare_f64(loc1.ploc(), &loc2["ploc"]);
    compare_f64(loc1.lloc(), &loc2["lloc"]);
    compare_f64(loc1.cloc(), &loc2["cloc"]);
    compare_f64(loc1.blank(), &loc2["blank"]);

    let nom1 = &metrics1.nom;
    let nom2 = &metrics2["nom"];
    compare_f64(nom1.functions_sum(), &nom2["functions"]);
    compare_f64(nom1.closures_sum(), &nom2["closures"]);
    compare_f64(nom1.total(), &nom2["total"]);
    compare_f64(nom1.functions_min(), &nom2["functions_min"]);
    compare_f64(nom1.functions_max(), &nom2["functions_max"]);
    compare_f64(nom1.closures_min(), &nom2["closures_min"]);
    compare_f64(nom1.closures_max(), &nom2["closures_max"]);

    let mi1 = &metrics1.mi;
    let mi2 = &metrics2["mi"];
    compare_f64(mi1.mi_original(), &mi2["mi_original"]);
    compare_f64(mi1.mi_sei(), &mi2["mi_sei"]);
    compare_f64(mi1.mi_visual_studio(), &mi2["mi_visual_studio"]);

    // Recursion
    for (pos, s) in funcspace.spaces.iter().enumerate() {
        compare_structs(s, &value["spaces"][pos]);
    }
}

/// Compares two f64 values truncated to 3 decimals
fn compare_f64(f1: f64, f2: &serde_json::Value) {
    if f1.is_nan() || f1.is_infinite() {
        assert!(f2.is_null());
    } else {
        let ft1 = f64::trunc(f1 * 1000.0) / 1000.0;
        let ft2 = f64::trunc(f2.as_f64().unwrap() * 1000.0) / 1000.0;
        assert!(ft1.total_cmp(&ft2) == std::cmp::Ordering::Equal);
    }
}

static REPO: Lazy<&Path> = Lazy::new(|| Path::new("./tests/repositories"));

/// Produces metrics runtime and compares them with previously generated json files
fn compare_rca_output_with_files(repo_name: &str, include: &[&str]) {
    let num_jobs = 4;

    let cfg = Config {
        language: None,
        output_folder: REPO.join("rca-output"),
    };

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
