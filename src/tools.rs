use crate::langs::fake;
use crate::langs::*;
use regex::bytes::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

/// Reads a file.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::read_file;
///
/// # fn main() {
/// let path = PathBuf::from("Cargo.toml");
/// read_file(&path).unwrap();
/// # }
/// ```
pub fn read_file(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

/// Reads a file and adds an `EOL` at its end.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::read_file_with_eol;
///
/// # fn main() {
/// let path = PathBuf::from("Cargo.toml");
/// read_file_with_eol(&path).unwrap();
/// # }
/// ```
pub fn read_file_with_eol(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();

    let mut bom = [0; 3];
    if let Ok(n) = file.read(&mut bom) {
        // Skip the bom if one
        if n == 3 {
            if bom[..2] == [b'\xFE', b'\xFF'] || bom[..2] == [b'\xFF', b'\xFE'] {
                data.push(bom[2]);
            } else if bom != [b'\xEF', b'\xBB', b'\xBF'] {
                data.extend_from_slice(&bom);
            }
        } else if n == 2 {
            if bom[..2] != [b'\xFE', b'\xFF'] && bom[..2] != [b'\xFF', b'\xFE'] {
                data.extend_from_slice(&bom[..2]);
            }
        } else if n == 1 {
            data.push(bom[0]);
        }
    }
    file.read_to_end(&mut data)?;

    if let Some(c) = data.last() {
        if *c != b'\n' {
            data.push(b'\n');
        }
    } else {
        data.push(b'\n');
    }

    Ok(data)
}

/// Writes data to a file.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use rust_code_analysis::write_file;
///
/// # fn main() {
/// let path = PathBuf::from("foo.txt");
/// let data: [u8; 4] = [0; 4];
/// write_file(&path, &data).unwrap();
/// # }
/// ```
pub fn write_file(path: &PathBuf, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;

    Ok(())
}

/// Detects the language of a code using
/// the extension of a file.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::get_language_for_file;
///
/// # fn main() {
/// let path = PathBuf::from("build.rs");
/// get_language_for_file(&path).unwrap();
/// # }
/// ```
pub fn get_language_for_file(path: &PathBuf) -> Option<LANG> {
    if let Some(ext) = path.extension() {
        let ext = ext.to_str().unwrap().to_lowercase();
        get_from_ext(&ext)
    } else {
        None
    }
}

fn mode_to_str(mode: &[u8]) -> Option<String> {
    std::str::from_utf8(mode).ok().map(|m| m.to_lowercase())
}

fn get_emacs_mode(buf: &[u8]) -> Option<String> {
    // we just try to use the emacs info (if there)
    lazy_static! {
        // comment containing coding info are useful
        static ref RE1_EMACS: Regex = Regex::new(r"(?i)-\*-.*[^-\w]mode\s*:\s*([^:;\s]+)").unwrap();
        static ref RE2_EMACS: Regex = Regex::new(r"-\*-\s*([^:;\s]+)\s*-\*-").unwrap();
        static ref RE1_VIM: Regex = Regex::new(r"(?i)vim\s*:.*[^\w]ft\s*=\s*([^:\s]+)").unwrap();
    }

    for (i, line) in buf.splitn(5, |c| *c == b'\n').enumerate() {
        if let Some(cap) = RE1_EMACS.captures_iter(line).next() {
            return mode_to_str(&cap[1]);
        } else if let Some(cap) = RE2_EMACS.captures_iter(line).next() {
            return mode_to_str(&cap[1]);
        } else if let Some(cap) = RE1_VIM.captures_iter(line).next() {
            return mode_to_str(&cap[1]);
        }
        if i == 3 {
            break;
        }
    }

    for (i, line) in buf.rsplitn(5, |c| *c == b'\n').enumerate() {
        if let Some(cap) = RE1_VIM.captures_iter(line).next() {
            return mode_to_str(&cap[1]);
        }
        if i == 3 {
            break;
        }
    }

    None
}

/// Guesses the language of a code.
///
/// Returns a tuple containing a [`LANG`] as first argument
/// and the language name as a second one.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::guess_language;
///
/// # fn main() {
/// let source_code = "int a = 42;";
///
/// // The path to a dummy file used to contain the source code
/// let path = PathBuf::from("foo.c");
/// let source_slice = source_code.as_bytes();
///
/// // Guess the language of a code
/// guess_language(&source_slice, &path);
/// # }
/// ```
///
/// [`LANG`]: enum.LANG.html
pub fn guess_language<P: AsRef<Path>>(buf: &[u8], path: P) -> (Option<LANG>, String) {
    let ext = path
        .as_ref()
        .extension()
        .map(|e| e.to_str().unwrap())
        .map(|e| e.to_lowercase())
        .unwrap_or_else(|| "".to_string());
    let from_ext = get_from_ext(&ext);

    let mode = get_emacs_mode(buf).unwrap_or_else(|| "".to_string());

    let from_mode = get_from_emacs_mode(&mode);

    if let Some(lang_ext) = from_ext {
        if let Some(lang_mode) = from_mode {
            if lang_ext == lang_mode {
                (
                    Some(lang_mode),
                    fake::get_true(&ext, &mode).unwrap_or_else(|| lang_mode.get_name().to_string()),
                )
            } else {
                // we should probably rely on extension here
                (Some(lang_ext), lang_ext.get_name().to_string())
            }
        } else {
            (
                Some(lang_ext),
                fake::get_true(&ext, &mode).unwrap_or_else(|| lang_ext.get_name().to_string()),
            )
        }
    } else if let Some(lang_mode) = from_mode {
        (
            Some(lang_mode),
            fake::get_true(&ext, &mode).unwrap_or_else(|| lang_mode.get_name().to_string()),
        )
    } else {
        (
            None,
            fake::get_true(&ext, &mode).unwrap_or_else(|| "".to_string()),
        )
    }
}

pub(crate) fn normalize_path<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Copied from Cargo sources: https://github.com/rust-lang/cargo/blob/master/src/cargo/util/paths.rs#L65
    let mut components = path.as_ref().components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    Some(ret)
}

pub(crate) fn get_paths_dist(path1: &PathBuf, path2: &PathBuf) -> Option<usize> {
    for ancestor in path1.ancestors() {
        if path2.starts_with(ancestor) && !ancestor.as_os_str().is_empty() {
            let path1 = path1.strip_prefix(ancestor).unwrap();
            let path2 = path2.strip_prefix(ancestor).unwrap();
            return Some(path1.components().count() + path2.components().count());
        }
    }
    None
}

pub(crate) fn guess_file<S: ::std::hash::BuildHasher>(
    current_path: &PathBuf,
    include_path: &str,
    all_files: &HashMap<String, Vec<PathBuf>, S>,
) -> Vec<PathBuf> {
    //let rpath = include_path.clone();
    let include_path = if include_path.starts_with("mozilla/") {
        &include_path[8..]
    } else {
        include_path
    };
    let include_path = normalize_path(include_path).unwrap();
    if let Some(possibilities) = all_files.get(include_path.file_name().unwrap().to_str().unwrap())
    {
        if possibilities.len() == 1 {
            // Only one file with this name
            return possibilities.clone();
        }

        let mut new_possibilities = Vec::new();
        for p in possibilities.iter() {
            if p.ends_with(&include_path) && current_path != p {
                new_possibilities.push(p.clone());
            }
        }
        if new_possibilities.len() == 1 {
            // Only one path is finishing with "foo/Bar.h"
            return new_possibilities;
        }
        new_possibilities.clear();

        if let Some(parent) = current_path.parent() {
            for p in possibilities.iter() {
                if p.starts_with(&parent) && current_path != p {
                    new_possibilities.push(p.clone());
                }
            }
            if new_possibilities.len() == 1 {
                // Only one path in the current working directory (current_path)
                return new_possibilities;
            }
            new_possibilities.clear();
        }

        let mut dist_min = std::usize::MAX;
        let mut path_min = Vec::new();
        for p in possibilities.iter() {
            if current_path == p {
                continue;
            }
            if let Some(dist) = get_paths_dist(current_path, &p) {
                match dist.cmp(&dist_min) {
                    Ordering::Less => {
                        dist_min = dist;
                        path_min.clear();
                        path_min.push(p);
                    }
                    Ordering::Equal => {
                        path_min.push(p);
                    }
                    Ordering::Greater => {}
                }
            }
        }

        let path_min: Vec<_> = path_min.drain(..).map(|p| p.to_path_buf()).collect();
        return path_min;
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_read() {
        let tmp_dir = std::env::temp_dir();
        let tmp_path = tmp_dir.join("test_read");
        let data = vec![
            (b"\xFF\xFEabc".to_vec(), b"abc\n".to_vec()),
            (b"\xFE\xFFabc".to_vec(), b"abc\n".to_vec()),
            (b"\xFE\xFF".to_vec(), b"\n".to_vec()),
            (b"\xFE".to_vec(), b"\xFE\n".to_vec()),
            (b"\xEF\xBB\xBFabc".to_vec(), b"abc\n".to_vec()),
            (b"\xEF\xBB\xBFabc\n".to_vec(), b"abc\n".to_vec()),
            (b"\xEF\xBBabc\n".to_vec(), b"\xEF\xBBabc\n".to_vec()),
            (b"abcdef\n".to_vec(), b"abcdef\n".to_vec()),
            (b"abcdef".to_vec(), b"abcdef\n".to_vec()),
            (b"ab".to_vec(), b"ab\n".to_vec()),
        ];
        for (d, expected) in data {
            write_file(&tmp_path, &d).unwrap();
            let res = read_file_with_eol(&tmp_path).unwrap();
            assert!(res == expected);
        }
    }

    #[test]
    fn test_guess_language() {
        let buf = b"// -*- foo: bar; mode: c++; hello: world\n";
        assert_eq!(
            guess_language(buf, "foo.cpp"),
            (Some(LANG::Cpp), "c/c++".to_string())
        );

        let buf = b"// -*- c++ -*-\n";
        assert_eq!(
            guess_language(buf, "foo.cpp"),
            (Some(LANG::Cpp), "c/c++".to_string())
        );

        let buf = b"// -*- foo: bar; bar-mode: c++; hello: world\n";
        assert_eq!(
            guess_language(buf, "foo.py"),
            (Some(LANG::Python), "python".to_string())
        );

        let buf = b"/* hello world */\n";
        assert_eq!(
            guess_language(buf, "foo.cpp"),
            (Some(LANG::Cpp), "c/c++".to_string())
        );

        let buf = b"\n\n\n\n\n\n\n\n\n// vim: set ts=4 ft=c++\n\n\n";
        assert_eq!(
            guess_language(buf, "foo.c"),
            (Some(LANG::Cpp), "c/c++".to_string())
        );

        let buf = b"\n\n\n\n\n\n\n\n\n\n\n\n";
        assert_eq!(guess_language(buf, "foo.txt"), (None, "".to_string()));

        let buf = b"// -*- foo: bar; mode: Objective-C++; hello: world\n";
        assert_eq!(
            guess_language(buf, "foo.mm"),
            (Some(LANG::Cpp), "obj-c/c++".to_string())
        );
    }
}
