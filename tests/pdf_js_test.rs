mod common;

use common::compare_rca_output_with_files;

#[test]
fn test_pdfjs() {
    compare_rca_output_with_files("pdf.js", &["*.js"]);
}

