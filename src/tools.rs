use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;

use regex::bytes::Regex;
use termcolor::{Color, ColorSpec, StandardStreamLock, WriteColor};

use crate::langs::fake;
use crate::langs::*;

/// Reads a file.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_code_analysis::read_file;
///
/// let path = Path::new("Cargo.toml");
/// read_file(&path).unwrap();
/// ```
pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    remove_blank_lines(&mut data);

    Ok(data)
}

/// Reads a file and adds an `EOL` at its end.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_code_analysis::read_file_with_eol;
///
/// let path = Path::new("Cargo.toml");
/// read_file_with_eol(&path).unwrap();
/// ```
pub fn read_file_with_eol(path: &Path) -> std::io::Result<Option<Vec<u8>>> {
    let file_size = fs::metadata(path).map_or(1024 * 1024, |m| m.len() as usize);
    if file_size <= 3 {
        // this file is very likely almost empty... so nothing to do on it
        return Ok(None);
    }

    let mut file = File::open(path)?;

    let mut start = vec![0; 64.min(file_size)];
    let start = if file.read_exact(&mut start).is_ok() {
        // Skip the bom if one
        if start[..2] == [b'\xFE', b'\xFF'] || start[..2] == [b'\xFF', b'\xFE'] {
            &start[2..]
        } else if start[..3] == [b'\xEF', b'\xBB', b'\xBF'] {
            &start[3..]
        } else {
            &start
        }
    } else {
        return Ok(None);
    };

    // so start contains more or less 64 chars
    let mut head = String::from_utf8_lossy(start).into_owned();
    // The last char could be wrong because we were in the middle of an utf-8 sequence
    head.pop();
    // now check if there is an invalid char
    if head.contains('\u{FFFD}') {
        return Ok(None);
    }

    let mut data = Vec::with_capacity(file_size + 2);
    data.extend_from_slice(start);

    file.read_to_end(&mut data)?;

    remove_blank_lines(&mut data);

    Ok(Some(data))
}

/// Writes data to a file.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// use rust_code_analysis::write_file;
///
/// let path = Path::new("foo.txt");
/// let data: [u8; 4] = [0; 4];
/// write_file(&path, &data).unwrap();
/// ```
pub fn write_file(path: &Path, data: &[u8]) -> std::io::Result<()> {
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
/// use std::path::Path;
///
/// use rust_code_analysis::get_language_for_file;
///
/// let path = Path::new("build.rs");
/// get_language_for_file(&path).unwrap();
/// ```
pub fn get_language_for_file(path: &Path) -> Option<LANG> {
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

// comment containing coding info are useful
static RE1_EMACS: OnceLock<Regex> = OnceLock::new();
static RE2_EMACS: OnceLock<Regex> = OnceLock::new();
static RE1_VIM: OnceLock<Regex> = OnceLock::new();

// Regular expressions
const FIRST_EMACS_EXPRESSION: &str = r"(?i)-\*-.*[^-\w]mode\s*:\s*([^:;\s]+)";
const SECOND_EMACS_EXPRESSION: &str = r"-\*-\s*([^:;\s]+)\s*-\*-";
const VIM_EXPRESSION: &str = r"(?i)vim\s*:.*[^\w]ft\s*=\s*([^:\s]+)";

#[inline(always)]
fn get_regex<'a>(
    once_lock: &OnceLock<Regex>,
    line: &'a [u8],
    regex: &'a str,
) -> Option<regex::bytes::Captures<'a>> {
    once_lock
        .get_or_init(|| Regex::new(regex).unwrap())
        .captures_iter(line)
        .next()
}

fn get_emacs_mode(buf: &[u8]) -> Option<String> {
    // we just try to use the emacs info (if there)
    for (i, line) in buf.splitn(5, |c| *c == b'\n').enumerate() {
        if let Some(cap) = get_regex(&RE1_EMACS, line, FIRST_EMACS_EXPRESSION) {
            return mode_to_str(&cap[1]);
        } else if let Some(cap) = get_regex(&RE2_EMACS, line, SECOND_EMACS_EXPRESSION) {
            return mode_to_str(&cap[1]);
        } else if let Some(cap) = get_regex(&RE1_VIM, line, VIM_EXPRESSION) {
            return mode_to_str(&cap[1]);
        }
        if i == 3 {
            break;
        }
    }

    for (i, line) in buf.rsplitn(5, |c| *c == b'\n').enumerate() {
        if let Some(cap) = get_regex(&RE1_VIM, line, VIM_EXPRESSION) {
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
/// let source_code = "int a = 42;";
///
/// // The path to a dummy file used to contain the source code
/// let path = PathBuf::from("foo.c");
/// let source_slice = source_code.as_bytes();
///
/// // Guess the language of a code
/// guess_language(&source_slice, &path);
/// ```
///
/// [`LANG`]: enum.LANG.html
pub fn guess_language<'a, P: AsRef<Path>>(buf: &[u8], path: P) -> (Option<LANG>, &'a str) {
    let ext = path
        .as_ref()
        .extension()
        .map(|e| e.to_str().unwrap())
        .map(|e| e.to_lowercase())
        .unwrap_or_else(|| "".to_string());
    let from_ext = get_from_ext(&ext);

    let mode = get_emacs_mode(buf).unwrap_or_default();

    let from_mode = get_from_emacs_mode(&mode);

    if let Some(lang_ext) = from_ext {
        if let Some(lang_mode) = from_mode {
            if lang_ext == lang_mode {
                (
                    Some(lang_mode),
                    fake::get_true(&ext, &mode).unwrap_or_else(|| lang_mode.get_name()),
                )
            } else {
                // we should probably rely on extension here
                (Some(lang_ext), lang_ext.get_name())
            }
        } else {
            (
                Some(lang_ext),
                fake::get_true(&ext, &mode).unwrap_or_else(|| lang_ext.get_name()),
            )
        }
    } else if let Some(lang_mode) = from_mode {
        (
            Some(lang_mode),
            fake::get_true(&ext, &mode).unwrap_or_else(|| lang_mode.get_name()),
        )
    } else {
        (None, fake::get_true(&ext, &mode).unwrap_or_default())
    }
}

/// Replaces \n and \r ending characters with a single generic \n
pub(crate) fn remove_blank_lines(data: &mut Vec<u8>) {
    let count_trailing = data
        .iter()
        .rev()
        .take_while(|&c| (*c == b'\n' || *c == b'\r'))
        .count();
    if count_trailing > 0 {
        data.truncate(data.len() - count_trailing);
    }
    data.push(b'\n');
}

pub(crate) fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
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
    ret
}

pub(crate) fn get_paths_dist(path1: &Path, path2: &Path) -> Option<usize> {
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
    current_path: &Path,
    include_path: &str,
    all_files: &HashMap<String, Vec<PathBuf>, S>,
) -> Vec<PathBuf> {
    let include_path = if let Some(end) = include_path.strip_prefix("mozilla/") {
        end
    } else {
        include_path
    };
    let include_path = normalize_path(include_path);
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
                if p.starts_with(parent) && current_path != p {
                    new_possibilities.push(p.clone());
                }
            }
            if new_possibilities.len() == 1 {
                // Only one path in the current working directory (current_path)
                return new_possibilities;
            }
            new_possibilities.clear();
        }

        let mut dist_min = usize::MAX;
        let mut path_min = Vec::new();
        for p in possibilities.iter() {
            if current_path == p {
                continue;
            }
            if let Some(dist) = get_paths_dist(current_path, p) {
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

#[inline(always)]
pub(crate) fn color(stdout: &mut StandardStreamLock, color: Color) -> std::io::Result<()> {
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))
}

#[inline(always)]
pub(crate) fn intense_color(stdout: &mut StandardStreamLock, color: Color) -> std::io::Result<()> {
    stdout.set_color(ColorSpec::new().set_fg(Some(color)).set_intense(true))
}

#[cfg(test)]
pub(crate) fn check_func_space<T: crate::ParserTrait, F: Fn(crate::FuncSpace)>(
    source: &str,
    filename: &str,
    check: F,
) {
    let path = std::path::PathBuf::from(filename);
    let mut trimmed_bytes = source.trim_end().trim_matches('\n').as_bytes().to_vec();
    trimmed_bytes.push(b'\n');
    let parser = T::new(trimmed_bytes, &path, None);
    let func_space = crate::metrics(&parser, &path).unwrap();

    check(func_space)
}

#[cfg(test)]
pub(crate) fn check_metrics<T: crate::ParserTrait>(
    source: &str,
    filename: &str,
    check: fn(crate::CodeMetrics) -> (),
) {
    check_func_space::<T, _>(source, filename, |func_space| check(func_space.metrics))
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
            (b"\xFF\xFEabc".to_vec(), Some(b"abc\n".to_vec())),
            (b"\xFE\xFFabc".to_vec(), Some(b"abc\n".to_vec())),
            (b"\xEF\xBB\xBFabc".to_vec(), Some(b"abc\n".to_vec())),
            (b"\xEF\xBB\xBFabc\n".to_vec(), Some(b"abc\n".to_vec())),
            (b"\xEF\xBBabc\n".to_vec(), None),
            (b"abcdef\n".to_vec(), Some(b"abcdef\n".to_vec())),
            (b"abcdef".to_vec(), Some(b"abcdef\n".to_vec())),
        ];
        for (d, expected) in data {
            write_file(&tmp_path, &d).unwrap();
            let res = read_file_with_eol(&tmp_path).unwrap();
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_guess_language() {
        let buf = b"// -*- foo: bar; mode: c++; hello: world\n";
        assert_eq!(guess_language(buf, "foo.cpp"), (Some(LANG::Cpp), "c/c++"));

        let buf = b"// -*- c++ -*-\n";
        assert_eq!(guess_language(buf, "foo.cpp"), (Some(LANG::Cpp), "c/c++"));

        let buf = b"// -*- foo: bar; bar-mode: c++; hello: world\n";
        assert_eq!(
            guess_language(buf, "foo.py"),
            (Some(LANG::Python), "python")
        );

        let buf = b"/* hello world */\n";
        assert_eq!(guess_language(buf, "foo.cpp"), (Some(LANG::Cpp), "c/c++"));

        let buf = b"\n\n\n\n\n\n\n\n\n// vim: set ts=4 ft=c++\n\n\n";
        assert_eq!(guess_language(buf, "foo.c"), (Some(LANG::Cpp), "c/c++"));

        let buf = b"\n\n\n\n\n\n\n\n\n\n\n\n";
        assert_eq!(guess_language(buf, "foo.txt"), (None, ""));

        let buf = b"// -*- foo: bar; mode: Objective-C++; hello: world\n";
        assert_eq!(
            guess_language(buf, "foo.mm"),
            (Some(LANG::Cpp), "obj-c/c++")
        );
    }
}
