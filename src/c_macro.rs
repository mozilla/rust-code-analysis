use std::collections::HashSet;

const DOLLARS: [u8; 2048] = [b'$'; 2048];
include!(concat!(env!("OUT_DIR"), "/gen_c_macros.rs"));

#[inline(always)]
fn is_identifier_part(c: u8) -> bool {
    (b'A' <= c && b'Z' >= c) || (b'a' <= c && b'z' >= c) || (b'0' <= c && b'9' >= c) || c == b'_'
}

#[inline(always)]
fn is_identifier_starter(c: u8) -> bool {
    (b'A' <= c && b'Z' >= c) || (b'a' <= c && b'z' >= c) || c == b'_'
}

#[inline(always)]
fn is_macro<S: ::std::hash::BuildHasher>(mac: &str, macros: &HashSet<String, S>) -> bool {
    macros.contains(mac) || PREDEFINED_MACROS.contains(mac)
}

pub fn replace<S: ::std::hash::BuildHasher>(
    code: &[u8],
    macros: &HashSet<String, S>,
) -> Option<Vec<u8>> {
    let mut new_code = Vec::with_capacity(code.len());
    let mut code_start = 0;
    let mut k_start = 0;

    for (i, c) in code.iter().enumerate() {
        if k_start != 0 {
            if !is_identifier_part(*c) {
                let start = k_start - 1;
                k_start = 0;
                let keyword = String::from_utf8(code[start..i].to_vec()).unwrap();
                if is_macro(&keyword, macros) {
                    new_code.extend(&code[code_start..start]);
                    new_code.extend(&DOLLARS[..(i - start)]);
                    code_start = i;
                }
            }
        } else if is_identifier_starter(*c) {
            k_start = i + 1;
        }
    }

    if k_start != 0 {
        let start = k_start - 1;
        let i = code.len();
        let keyword = String::from_utf8(code[start..].to_vec()).unwrap();
        if is_macro(&keyword, macros) {
            new_code.extend(&code[code_start..start]);
            new_code.extend(&DOLLARS[..(i - start)]);
            code_start = i;
        }
    }

    if code_start == 0 {
        None
    } else {
        if code_start < code.len() {
            new_code.extend(&code[code_start..]);
        }
        Some(new_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let mut mac = HashSet::new();
        mac.insert("abc".to_string());

        assert!(replace("def ghi jkl".as_bytes(), &mac).is_none());
        assert!(
            "$$$ def ghi jkl".as_bytes().to_vec()
                == replace("abc def ghi jkl".as_bytes(), &mac).unwrap()
        );
        assert!(
            "def $$$ ghi jkl".as_bytes().to_vec()
                == replace("def abc ghi jkl".as_bytes(), &mac).unwrap()
        );
        assert!(
            "def ghi $$$ jkl".as_bytes().to_vec()
                == replace("def ghi abc jkl".as_bytes(), &mac).unwrap()
        );
        assert!(
            "def ghi jkl $$$".as_bytes().to_vec()
                == replace("def ghi jkl abc".as_bytes(), &mac).unwrap()
        );

        mac.insert("z9_".to_string());
        assert!(
            "$$$ def ghi $$$ jkl".as_bytes().to_vec()
                == replace("abc def ghi z9_ jkl".as_bytes(), &mac).unwrap()
        );
    }
}
