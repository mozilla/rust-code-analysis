extern crate phf_codegen;

use askama::Template;
use enum_iterator::IntoEnumIterator;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use crate::common::*;
use crate::languages::*;

const MACROS_DEFINITION_DIR: &str = "data";

#[derive(Template)]
#[template(path = "rust.rs", escape = "none")]
struct RustTemplate {
    c_name: String,
    names: Vec<(String, bool, String)>,
}

pub fn generate_rust(output: &str, file_template: &str) -> std::io::Result<()> {
    for lang in Lang::into_enum_iter() {
        let language = get_language(&lang);
        let name = get_language_name(&lang);
        let c_name = camel_case(name.to_string());

        let file_name = format!("{}.rs", file_template.replace("$", &c_name.to_lowercase()));
        let path = PathBuf::from(output).join(file_name);
        let mut file = File::create(path)?;

        let names = get_token_names(&language, false);

        let args = RustTemplate { c_name, names };

        file.write_all(args.render().unwrap().as_bytes())?;
    }

    Ok(())
}

pub fn generate_macros(output: &str) -> std::io::Result<()> {
    create_macros_file(output, "c_macros", "PREDEFINED_MACROS")?;
    create_macros_file(output, "c_specials", "SPECIALS")
}

fn create_macros_file(output: &str, data_name: &str, set_name: &str) -> std::io::Result<()> {
    let mut set = phf_codegen::Set::new();
    let mut file = File::open(PathBuf::from(format!(
        "{}/{}/{}.txt",
        &env::var("CARGO_MANIFEST_DIR").unwrap(),
        MACROS_DEFINITION_DIR,
        data_name
    )))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    for tok in data.split(|c| *c == b'\n') {
        let tok = std::str::from_utf8(tok).unwrap().trim();
        if !tok.is_empty() {
            set.entry(tok);
        }
    }
    let path = Path::new(output).join(format!("{}.rs", data_name));
    let mut file = BufWriter::new(File::create(&path)?);
    writeln!(&mut file, "#[allow(clippy::unreadable_literal)]").unwrap();
    writeln!(
        &mut file,
        "static {}: phf::Set<&'static str> =\n{};\n",
        set_name,
        set.build()
    )?;
    writeln!(
        &mut file,
        "pub fn is_{}(mac: &str) -> bool {{ {}.contains(mac) }}\n",
        set_name.to_lowercase(),
        set_name,
    )
}
