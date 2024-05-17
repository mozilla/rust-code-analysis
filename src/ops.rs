use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::checker::Checker;
use crate::getter::Getter;
use crate::node::Node;
use crate::spaces::SpaceKind;

use crate::halstead::{Halstead, HalsteadMaps};

use crate::dump_ops::*;
use crate::traits::*;

/// All operands and operators of a space.
#[derive(Debug, Clone, Serialize)]
pub struct Ops {
    /// The name of a function space.
    ///
    /// If `None`, an error is occured in parsing
    /// the name of a function space.
    pub name: Option<String>,
    /// The first line of a function space.
    pub start_line: usize,
    /// The last line of a function space.
    pub end_line: usize,
    /// The space kind.
    pub kind: SpaceKind,
    /// All subspaces contained in a function space.
    pub spaces: Vec<Ops>,
    /// All operands of a space.
    pub operands: Vec<String>,
    /// All operators of a space.
    pub operators: Vec<String>,
}

impl Ops {
    fn new<T: Getter>(node: &Node, code: &[u8], kind: SpaceKind) -> Self {
        let (start_position, end_position) = match kind {
            SpaceKind::Unit => {
                if node.child_count() == 0 {
                    (0, 0)
                } else {
                    (node.start_row() + 1, node.end_row())
                }
            }
            _ => (node.start_row() + 1, node.end_row() + 1),
        };
        Self {
            name: T::get_func_space_name(node, code).map(|name| name.to_string()),
            spaces: Vec::new(),
            kind,
            start_line: start_position,
            end_line: end_position,
            operators: Vec::new(),
            operands: Vec::new(),
        }
    }

    pub(crate) fn merge_ops(&mut self, other: &Ops) {
        self.operands.extend_from_slice(&other.operands);
        self.operators.extend_from_slice(&other.operators);
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    ops: Ops,
    halstead_maps: HalsteadMaps<'a>,
    primitive_types: HashSet<String>,
}

fn compute_operators_and_operands<T: ParserTrait>(state: &mut State) {
    state.ops.operators = state
        .halstead_maps
        .operators
        .keys()
        .filter(|k| !T::Checker::is_primitive(**k))
        .map(|k| T::Getter::get_operator_id_as_str(*k).to_owned())
        .collect();

    // Add primitive types to operators
    let v: Vec<_> = state.primitive_types.iter().cloned().collect();
    state.ops.operators.extend_from_slice(&v);
    println!("{:?}", state.ops.operators);
    println!("{:?}", state.halstead_maps.operators);

    state.ops.operands = state
        .halstead_maps
        .operands
        .keys()
        .map(|k| String::from_utf8(k.to_vec()).unwrap_or_else(|_| String::from("wrong_operands")))
        .collect();
}

fn finalize<T: ParserTrait>(state_stack: &mut Vec<State>, diff_level: usize) {
    if state_stack.is_empty() {
        return;
    }

    // If there is only the unit space
    if state_stack.len() == 1 {
        let last_state = state_stack.last_mut().unwrap();
        // Compute last_state operators and operands
        compute_operators_and_operands::<T>(last_state);
    }

    for _ in 0..diff_level {
        if state_stack.len() == 1 {
            break;
        } else {
            let mut state = state_stack.pop().unwrap();
            let last_state = state_stack.last_mut().unwrap();

            // Compute state operators and operands
            compute_operators_and_operands::<T>(&mut state);

            // Compute last_state operators and operands
            compute_operators_and_operands::<T>(last_state);

            // Merge Halstead maps
            last_state.halstead_maps.merge(&state.halstead_maps);

            // Merge operands and operators between spaces
            last_state.ops.merge_ops(&state.ops);
            last_state.ops.spaces.push(state.ops);
        }
    }
}

/// Retrieves all the operators and operands of a code.
///
/// If `None`, it was not possible to retrieve the operators and operands
/// of a code.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::{operands_and_operators, CppParser, ParserTrait};
///
/// # fn main() {
/// let source_code = "int a = 42;";
///
/// // The path to a dummy file used to contain the source code
/// let path = PathBuf::from("foo.c");
/// let source_as_vec = source_code.as_bytes().to_vec();
///
/// // The parser of the code, in this case a CPP parser
/// let parser = CppParser::new(source_as_vec, &path, None);
///
/// // Returns the operands and operators of each space in a code.
/// operands_and_operators(&parser, &path).unwrap();
/// # }
/// ```
pub fn operands_and_operators<'a, T: ParserTrait>(parser: &'a T, path: &'a Path) -> Option<Ops> {
    let code = parser.get_code();
    let node = parser.get_root();
    let mut cursor = node.cursor();
    let mut stack = Vec::new();
    let mut children = Vec::new();
    let mut state_stack: Vec<State> = Vec::new();
    let mut last_level = 0;

    stack.push((node, 0));

    while let Some((node, level)) = stack.pop() {
        if level < last_level {
            finalize::<T>(&mut state_stack, last_level - level);
            last_level = level;
        }

        let kind = T::Getter::get_space_kind(&node);

        let func_space = T::Checker::is_func(&node) || T::Checker::is_func_space(&node);

        let new_level = if func_space {
            let state = State {
                ops: Ops::new::<T::Getter>(&node, code, kind),
                halstead_maps: HalsteadMaps::new(),
                primitive_types: HashSet::new(),
            };
            state_stack.push(state);
            last_level = level + 1;
            last_level
        } else {
            level
        };

        if let Some(state) = state_stack.last_mut() {
            T::Halstead::compute(&node, code, &mut state.halstead_maps);
            if T::Checker::is_primitive(node.kind_id()) {
                let code = &code[node.start_byte()..node.end_byte()];
                let primitive_string = String::from_utf8(code.to_vec())
                    .unwrap_or_else(|_| String::from("primitive_type"));
                state.primitive_types.insert(primitive_string);
            }
        }

        cursor.reset(&node);
        if cursor.goto_first_child() {
            loop {
                children.push((cursor.node(), new_level));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            for child in children.drain(..).rev() {
                stack.push(child);
            }
        }
    }

    finalize::<T>(&mut state_stack, std::usize::MAX);

    state_stack.pop().map(|mut state| {
        state.ops.name = path.to_str().map(|name| name.to_string());
        state.ops
    })
}

/// Configuration options for retrieving
/// all the operands and operators in a code.
#[derive(Debug)]
pub struct OpsCfg {
    /// Path to the file containing the code.
    pub path: PathBuf,
}

pub struct OpsCode {
    _guard: (),
}

impl Callback for OpsCode {
    type Res = std::io::Result<()>;
    type Cfg = OpsCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(ops) = operands_and_operators(parser, &cfg.path) {
            dump_ops(&ops)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{get_ops, LANG};

    #[inline(always)]
    fn check_ops(
        lang: LANG,
        source: &str,
        file: &str,
        correct_operators: &mut [&str],
        correct_operands: &mut [&str],
    ) {
        let path = PathBuf::from(file);
        let mut trimmed_bytes = source.trim_end().trim_matches('\n').as_bytes().to_vec();
        trimmed_bytes.push(b'\n');
        let ops = get_ops(&lang, trimmed_bytes, &path, None).unwrap();

        let mut operators_str: Vec<&str> = ops.operators.iter().map(AsRef::as_ref).collect();
        let mut operands_str: Vec<&str> = ops.operands.iter().map(AsRef::as_ref).collect();

        // Sorting out operators because they are returned in arbitrary order
        operators_str.sort_unstable();
        correct_operators.sort_unstable();

        assert_eq!(&operators_str[..], correct_operators);

        // Sorting out operands because they are returned in arbitrary order
        operands_str.sort_unstable();
        correct_operands.sort_unstable();

        assert_eq!(&operands_str[..], correct_operands);
    }

    #[test]
    fn python_ops() {
        check_ops(
            LANG::Python,
            "if True:
                 a = 1 + 2",
            "foo.py",
            &mut ["if", "=", "+"],
            &mut ["True", "a", "1", "2"],
        );
    }

    #[test]
    fn python_function_ops() {
        check_ops(
            LANG::Python,
            "def foo():
                 def bar():
                     def toto():
                        a = 1 + 1
                     b = 2 + a
                 c = 3 + 3",
            "foo.py",
            &mut ["def", "=", "+"],
            &mut ["foo", "bar", "toto", "a", "b", "c", "1", "2", "3"],
        );
    }

    #[test]
    fn cpp_ops() {
        check_ops(
            LANG::Cpp,
            "int a, b, c;
             float avg;
             avg = (a + b + c) / 3;",
            "foo.c",
            &mut ["int", "float", "()", "=", "+", "/", ",", ";"],
            &mut ["a", "b", "c", "avg", "3"],
        );
    }

    #[test]
    fn cpp_function_ops() {
        check_ops(
            LANG::Cpp,
            "main()
            {
              int a, b, c, avg;
              scanf(\"%d %d %d\", &a, &b, &c);
              avg = (a + b + c) / 3;
              printf(\"avg = %d\", avg);
            }",
            "foo.c",
            &mut ["()", "{}", "int", "&", "=", "+", "/", ",", ";"],
            &mut [
                "main",
                "a",
                "b",
                "c",
                "avg",
                "scanf",
                "\"%d %d %d\"",
                "3",
                "printf",
                "\"avg = %d\"",
            ],
        );
    }

    #[test]
    fn rust_ops() {
        check_ops(
            LANG::Rust,
            "let: usize a = 5; let b: f32 = 7.0; let c: i32 = 3;",
            "foo.rs",
            // FIXME tree-sitter-rust does not parse the comma inside the println! macro
            &mut ["let", "usize", "=", ";", "f32", "i32"],
            &mut ["a", "b", "c", "5", "7.0", "3"],
        );
    }

    #[test]
    fn rust_function_ops() {
        check_ops(
            LANG::Rust,
            "fn main() {
              let a = 5; let b = 5; let c = 5;
              let avg = (a + b + c) / 3;
              println!(\"{}\", avg);
            }",
            "foo.rs",
            // FIXME tree-sitter-rust does not parse the comma inside the println! macro
            &mut ["fn", "()", "{}", "let", "=", "+", "/", ";", "!"],
            &mut ["main", "a", "b", "c", "avg", "5", "3", "println", "\"{}\""],
        );
    }

    #[test]
    fn javascript_ops() {
        check_ops(
            LANG::Javascript,
            "var a, b, c, avg;
             let x = 1;
             a = 5; b = 5; c = 5;
             avg = (a + b + c) / 3;
             console.log(\"{}\", avg);",
            "foo.js",
            &mut ["()", "var", "let", "=", "+", "/", ",", ".", ";"],
            &mut [
                "a",
                "b",
                "c",
                "avg",
                "x",
                "1",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn javascript_function_ops() {
        check_ops(
            LANG::Javascript,
            "function main() {
              var a, b, c, avg;
              let x = 1;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.js",
            &mut [
                "function", "()", "{}", "var", "let", "=", "+", "/", ",", ".", ";",
            ],
            &mut [
                "main",
                "a",
                "b",
                "c",
                "avg",
                "x",
                "1",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn mozjs_ops() {
        check_ops(
            LANG::Mozjs,
            "var a, b, c, avg;
             let x = 1;
             a = 5; b = 5; c = 5;
             avg = (a + b + c) / 3;
             console.log(\"{}\", avg);",
            "foo.js",
            &mut ["()", "var", "let", "=", "+", "/", ",", ".", ";"],
            &mut [
                "a",
                "b",
                "c",
                "avg",
                "x",
                "1",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn mozjs_function_ops() {
        check_ops(
            LANG::Mozjs,
            "function main() {
              var a, b, c, avg;
              let x = 1;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.js",
            &mut [
                "function", "()", "{}", "var", "let", "=", "+", "/", ",", ".", ";",
            ],
            &mut [
                "main",
                "a",
                "b",
                "c",
                "avg",
                "x",
                "1",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn typescript_ops() {
        check_ops(
            LANG::Typescript,
            "var a, b, c, avg;
             let age: number = 32;
             let name: string = \"John\"; let isUpdated: boolean = true;
             a = 5; b = 5; c = 5;
             avg = (a + b + c) / 3;
             console.log(\"{}\", avg);",
            "foo.ts",
            &mut [
                "()", "var", "let", "string", "number", "boolean", ":", "=", "+", "/", ",", ".",
                ";",
            ],
            &mut [
                "a",
                "b",
                "c",
                "avg",
                "age",
                "name",
                "isUpdated",
                "32",
                "\"John\"",
                "true",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn typescript_function_ops() {
        check_ops(
            LANG::Typescript,
            "function main() {
              var a, b, c, avg;
              let age: number = 32;
              let name: string = \"John\"; let isUpdated: boolean = true;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.ts",
            &mut [
                "function", "()", "{}", "var", "let", "string", "number", "boolean", ":", "=", "+",
                "/", ",", ".", ";",
            ],
            &mut [
                "main",
                "a",
                "b",
                "c",
                "avg",
                "age",
                "name",
                "isUpdated",
                "32",
                "\"John\"",
                "true",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn tsx_ops() {
        check_ops(
            LANG::Tsx,
            "var a, b, c, avg;
             let age: number = 32;
             let name: string = \"John\"; let isUpdated: boolean = true;
             a = 5; b = 5; c = 5;
             avg = (a + b + c) / 3;
             console.log(\"{}\", avg);",
            "foo.ts",
            &mut [
                "()", "var", "let", "string", "number", "boolean", ":", "=", "+", "/", ",", ".",
                ";",
            ],
            &mut [
                "a",
                "b",
                "c",
                "avg",
                "age",
                "name",
                "isUpdated",
                "32",
                "\"John\"",
                "true",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn tsx_function_ops() {
        check_ops(
            LANG::Tsx,
            "function main() {
              var a, b, c, avg;
              let age: number = 32;
              let name: string = \"John\"; let isUpdated: boolean = true;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.ts",
            &mut [
                "function", "()", "{}", "var", "let", "string", "number", "boolean", ":", "=", "+",
                "/", ",", ".", ";",
            ],
            &mut [
                "main",
                "a",
                "b",
                "c",
                "avg",
                "age",
                "name",
                "isUpdated",
                "32",
                "\"John\"",
                "true",
                "3",
                "5",
                "console.log",
                "console",
                "log",
                "\"{}\"",
            ],
        );
    }

    #[test]
    fn java_ops() {
        check_ops(
            LANG::Java,
            "public class Main {
                public static void main(string args[]) {
                      int a, b, c, avg;
                      a = 5; b = 5; c = 5;
                      avg = (a + b + c) / 3;
                      MessageFormat.format(\"{0}\", avg);
                    }
                }",
            "foo.java",
            &mut ["{}", "void", "()", "[]", ",", ";", "int", "=", "+", "/"],
            &mut [
                "Main",
                "main",
                "args",
                "a",
                "b",
                "c",
                "avg",
                "5",
                "3",
                "MessageFormat",
                "format",
                "\"{0}\"",
            ],
        );
    }
}
