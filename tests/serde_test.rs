mod common;

use common::compare_rca_output_with_files;

#[test]
fn test_serde() {
    compare_rca_output_with_files("serde", &["*.rs"]);
}
