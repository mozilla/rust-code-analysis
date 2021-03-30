extern crate phf_codegen;

use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn mk_predef(data_name: &str, set_name: &str) {
    let mut set = phf_codegen::Set::new();
    let mut file = fs::File::open(PathBuf::from(format!("./data/{}.txt", data_name))).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    for tok in data.split(|c| *c == b'\n') {
        let tok = std::str::from_utf8(tok).unwrap().trim();
        if !tok.is_empty() {
            set.entry(tok);
        }
    }
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("gen_{}.rs", data_name));
    let mut file = BufWriter::new(fs::File::create(&path).unwrap());
    writeln!(&mut file, "#[allow(clippy::unreadable_literal)]").unwrap();
    writeln!(
        &mut file,
        "static {}: phf::Set<&'static str> =\n{};\n",
        set_name,
        set.build()
    )
    .unwrap();
}

fn main() {
    mk_predef("c_macros", "PREDEFINED_MACROS");
    mk_predef("c_specials", "SPECIALS");
}
