extern crate phf_codegen;

use askama::Template;
use enum_iterator::IntoEnumIterator;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::PathBuf;

use crate::common::*;
use crate::languages::*;

#[derive(Template)]
#[template(path = "rust.rs", escape = "none")]
struct RustTemplate {
    c_name: String,
    names: Vec<(String, bool, String)>,
    phf_map: String,
}

pub fn generate_rust(output: &str, file_template: &str) -> std::io::Result<()> {
    for lang in LANG::into_enum_iter() {
        let language = get_language(&lang);
        let name = get_language_name(&lang);
        let c_name = camel_case(name.to_string());

        let file_name = format!("{}.rs", file_template.replace("$", &c_name.to_lowercase()));
        let path = PathBuf::from(output).join(file_name);
        let mut file = File::create(path)?;

        let names = get_token_names(&language, false);

        let mut phf_map = Cursor::new(Vec::new());
        let mut builder = phf_codegen::Map::new();
        for (name, dup, ts_name) in names.iter() {
            if !dup {
                builder.entry(ts_name.as_str(), format!("{}::{}", c_name, name).as_str());
            }
        }
        writeln!(phf_map, "{}", builder.build()).unwrap();
        let phf_map = String::from_utf8(phf_map.into_inner()).unwrap();

        let args = RustTemplate {
            c_name,
            names,
            phf_map,
        };

        file.write_all(args.render().unwrap().as_bytes())?;
    }

    Ok(())
}
