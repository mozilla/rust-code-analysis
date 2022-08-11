use askama::Template;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::common::*;
use crate::languages::*;

const MACROS_DEFINITION_DIR: &str = "data";

#[derive(Template)]
#[template(path = "rust.rs", escape = "none")]
struct RustTemplate {
    c_name: String,
    names: Vec<(String, bool, String)>,
}

pub fn generate_rust(output: &Path, file_template: &str) -> std::io::Result<()> {
    for lang in Lang::into_enum_iter() {
        let language = get_language(&lang);
        let name = get_language_name(&lang);
        let c_name = camel_case(name.to_string());

        let file_name = format!("{}.rs", file_template.replace('$', &c_name.to_lowercase()));
        let path = output.join(file_name);
        let mut file = File::create(path)?;

        let names = get_token_names(&language, false);

        let args = RustTemplate { c_name, names };

        file.write_all(args.render().unwrap().as_bytes())?;
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "c_macros.rs", escape = "none")]
struct CMacrosTemplate {
    u_name: String,
    l_name: String,
    names: Vec<String>,
}

pub fn generate_macros(output: &Path) -> std::io::Result<()> {
    create_macros_file(output, "c_macros", "PREDEFINED_MACROS")?;
    create_macros_file(output, "c_specials", "SPECIALS")
}

fn create_macros_file(output: &Path, filename: &str, u_name: &str) -> std::io::Result<()> {
    let mut macro_file = File::open(Path::new(&format!(
        "{}/{}/{}.txt",
        &env::var("CARGO_MANIFEST_DIR").unwrap(),
        MACROS_DEFINITION_DIR,
        filename
    )))?;
    let mut data = Vec::new();
    macro_file.read_to_end(&mut data)?;

    let mut names = Vec::new();
    for tok in data.split(|c| *c == b'\n') {
        let tok = std::str::from_utf8(tok).unwrap().trim();
        if !tok.is_empty() {
            names.push(tok.to_owned());
        }
    }
    let l_name = u_name.to_lowercase();

    let path = output.join(format!("{}.rs", filename));

    let mut file = File::create(&path)?;

    let args = CMacrosTemplate {
        u_name: u_name.to_owned(),
        l_name,
        names,
    };

    file.write_all(args.render().unwrap().as_bytes())
}
