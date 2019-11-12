use crate::languages::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

pub fn read_file(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

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

pub fn write_file(path: &PathBuf, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;

    Ok(())
}

pub fn get_language_for_file(path: &PathBuf) -> Option<LANG> {
    if let Some(ext) = path.extension() {
        let ext = ext.to_str().unwrap().to_lowercase();
        get_from_ext(&ext)
    } else {
        None
    }
}
pub fn normalize_path<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
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

pub fn get_paths_dist(path1: &PathBuf, path2: &PathBuf) -> Option<usize> {
    for ancestor in path1.ancestors() {
        if path2.starts_with(ancestor) && !ancestor.as_os_str().is_empty() {
            let path1 = path1.strip_prefix(ancestor).unwrap();
            let path2 = path2.strip_prefix(ancestor).unwrap();
            return Some(path1.components().count() + path2.components().count());
        }
    }
    None
}

pub fn guess_file(
    current_path: &PathBuf,
    include_path: &str,
    all_files: &HashMap<String, Vec<PathBuf>>,
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
                if dist < dist_min {
                    dist_min = dist;
                    path_min.clear();
                    path_min.push(p);
                } else if dist == dist_min {
                    path_min.push(p)
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
}
