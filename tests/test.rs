mod common;

use common::compare_rca_output_with_files;

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
