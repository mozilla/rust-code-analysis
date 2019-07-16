use askama::Template;
use enum_iterator::IntoEnumIterator;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::common::*;
use crate::languages::*;

#[derive(Template)]
#[template(path = "json.json", escape = "none")]
struct JsonTemplate {
    names: Vec<(String, bool, String)>,
}

pub fn generate_json(output: &str, file_template: &str) -> std::io::Result<()> {
    for lang in LANG::into_enum_iter() {
        let language = get_language(&lang);
        let name = get_language_name(&lang);
        let c_name = camel_case(name.to_string());

        let file_name = format!(
            "{}.json",
            file_template.replace("$", &c_name.to_lowercase())
        );
        let path = PathBuf::from(output).join(file_name);
        let mut file = File::create(path)?;

        let names = get_token_names(&language, true);

        let args = JsonTemplate { names };

        file.write_all(args.render().unwrap().as_bytes())?;
    }

    Ok(())
}
