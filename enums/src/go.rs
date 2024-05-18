use askama::Template;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::common::*;
use crate::languages::*;

#[derive(Debug, Template)]
#[template(path = "go.go", escape = "none")]
struct GoTemplate {
    c_name: String,
    names: Vec<(String, bool, String, String)>,
}

pub fn generate_go(output: &Path, file_template: &str) -> std::io::Result<()> {
    for lang in Lang::into_enum_iter() {
        let language = get_language(&lang);
        let name = get_language_name(&lang);
        let c_name = camel_case(name.to_string());

        let file_name = format!("{}.go", file_template.replace('$', &c_name.to_lowercase()));
        let path = output.join(file_name);
        let mut file = File::create(path)?;

        let mut names = get_token_names(&language, false);
        let max_len = names.iter().map(|x| x.0.len()).max().unwrap();
        let names: Vec<_> = names
            .drain(..)
            .map(move |(n, d, t)| (n.clone(), d, t, format!("{: <width$}", n, width = max_len)))
            .collect();

        let args = GoTemplate { c_name, names };

        file.write_all(args.render().unwrap().as_bytes())?;
    }

    Ok(())
}
