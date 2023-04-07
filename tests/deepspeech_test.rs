mod common;

use common::compare_rca_output_with_files;

#[test]
fn test_deepspeech() {
    compare_rca_output_with_files("DeepSpeech", &["*.cc", "*.cpp", "*.h", "*.hh"]);
}
