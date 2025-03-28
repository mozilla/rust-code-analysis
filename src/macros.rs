macro_rules! get_language {
    (tree_sitter_cpp) => {
        tree_sitter_mozcpp::language()
    };
    (tree_sitter_typescript) => {
        tree_sitter_typescript::language_typescript()
    };
    (tree_sitter_tsx) => {
        tree_sitter_typescript::language_tsx()
    };
    ($name:ident) => {
        $name::LANGUAGE.into()
    };
}

macro_rules! implement_metric_trait {
    (Abc, $($code:ident),+) => (
        $(
           impl Abc for $code {
               fn compute(_node: &Node, _stats: &mut Stats) {}
           }
        )+
    );
    (Cognitive, $($code:ident),+) => (
        $(
           impl Cognitive for $code {
               fn compute(_node: &Node, _stats: &mut Stats, _nesting_map: &mut HashMap<usize, (usize, usize, usize)>,) {}
           }
        )+
    );
    (Halstead, $($code:ident),+) => (
        $(
           impl Halstead for $code {
               fn compute<'a>(_node: &Node<'a>, _code: &'a [u8], _halstead_maps: &mut HalsteadMaps<'a>) {}
           }
        )+
    );
    (Loc, $($code:ident),+) => (
        $(
           impl Loc for $code {
               fn compute(_node: &Node, _stats: &mut Stats, _is_func_space: bool, _is_unit: bool) {}
           }
        )+
    );
    (Wmc, $($code:ident),+) => (
        $(
           impl Wmc for $code {
               fn compute(_space_kind: SpaceKind, _cyclomatic: &cyclomatic::Stats, _stats: &mut Stats) {}
           }
        )+
    );
    ([$trait:ident], $($code:ident),+) => (
        $(
           impl $trait for $code {}
        )+
    );
    ($trait:ident, $($code:ident),+) => (
        $(
           impl $trait for $code {
               fn compute(_node: &Node, _stats: &mut Stats) {}
           }
        )+
    )
}

macro_rules! mk_lang {
    ( $( ($camel:ident, $name:ident, $display: expr, $description:expr) ),* ) => {
        /// The list of supported languages.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum LANG {
            $(
                #[doc = $description]
                $camel,
            )*
        }
        impl LANG {
            pub fn into_enum_iter() -> impl Iterator<Item=LANG> {
                use LANG::*;
                [$( $camel, )*].into_iter()
            }

            /// Returns the name of a language as a `&str`.
            ///
            /// # Examples
            ///
            /// ```
            /// use rust_code_analysis::LANG;
            ///
            /// println!("{}", LANG::Rust.get_name());
            /// ```
            pub fn get_name(&self) -> &'static str {
                match self {
                    $(
                        LANG::$camel => $display,
                    )*
                }
            }

            // Returns a tree-sitter language.
            // This function is only used to construct a parser.
            pub(crate) fn get_ts_language(&self) -> Language {
                    match self {
                        $(
                            LANG::$camel => get_language!($name),
                        )*
                    }
            }
        }
    };
}

macro_rules! mk_action {
    ( $( ($camel:ident, $parser:ident) ),* ) => {
        /// Runs a function, which implements the [`Callback`] trait,
        /// on a code written in one of the supported languages.
        ///
        /// # Examples
        ///
        /// The following example dumps to shell every metric computed using
        /// the dummy source code.
        ///
        /// ```
        /// use std::path::PathBuf;
        ///
        /// use rust_code_analysis::{action, Callback, LANG, Metrics, MetricsCfg};
        ///
        /// let source_code = "int a = 42;";
        /// let language = LANG::Cpp;
        ///
        /// // The path to a dummy file used to contain the source code
        /// let path = PathBuf::from("foo.c");
        /// let source_as_vec = source_code.as_bytes().to_vec();
        ///
        /// // Configuration options used by the function which computes the metrics
        /// let cfg = MetricsCfg {
        ///     path,
        /// };
        ///
        /// action::<Metrics>(&language, source_as_vec, &cfg.path.clone(), None, cfg);
        /// ```
        ///
        /// [`Callback`]: trait.Callback.html
        #[inline(always)]
        pub fn action<T: Callback>(lang: &LANG, source: Vec<u8>, path: &Path, pr: Option<Arc<PreprocResults>>, cfg: T::Cfg) -> T::Res {
            match lang {
                $(
                    LANG::$camel => {
                        let parser = $parser::new(source, path, pr);
                        T::call(cfg, &parser)
                    },
                )*
            }
        }

        /// Returns all function spaces data of a code.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::path::PathBuf;
        ///
        /// use rust_code_analysis::{get_function_spaces, LANG};
        ///
        /// let source_code = "int a = 42;";
        /// let language = LANG::Cpp;
        ///
        /// // The path to a dummy file used to contain the source code
        /// let path = PathBuf::from("foo.c");
        /// let source_as_vec = source_code.as_bytes().to_vec();
        ///
        /// get_function_spaces(&language, source_as_vec, &path, None).unwrap();
        /// ```
        #[inline(always)]
        pub fn get_function_spaces(lang: &LANG, source: Vec<u8>, path: &Path, pr: Option<Arc<PreprocResults>>) -> Option<FuncSpace> {
            match lang {
                $(
                    LANG::$camel => {
                        let parser = $parser::new(source, &path, pr);
                        metrics(&parser, &path)
                    },
                )*
            }
        }

        /// Returns all operators and operands of each space in a code.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::path::PathBuf;
        ///
        /// use rust_code_analysis::{get_ops, LANG};
        ///
        /// # fn main() {
        /// let source_code = "int a = 42;";
        /// let language = LANG::Cpp;
        ///
        /// // The path to a dummy file used to contain the source code
        /// let path = PathBuf::from("foo.c");
        /// let source_as_vec = source_code.as_bytes().to_vec();
        ///
        /// get_ops(&language, source_as_vec, &path, None).unwrap();
        /// # }
        /// ```
        #[inline(always)]
        pub fn get_ops(lang: &LANG, source: Vec<u8>, path: &Path, pr: Option<Arc<PreprocResults>>) -> Option<Ops> {
            match lang {
                $(
                    LANG::$camel => {
                        let parser = $parser::new(source, &path, pr);
                        operands_and_operators(&parser, &path)
                    },
                )*
            }
        }
    };
}

macro_rules! mk_extensions {
    ( $( ($camel:ident, [ $( $ext:ident ),* ]) ),* ) => {
        /// Detects the language associated to the input file extension.
        ///
        /// # Examples
        ///
        /// ```
        /// use rust_code_analysis::get_from_ext;
        ///
        /// let ext = "rs";
        ///
        /// get_from_ext(ext).unwrap();
        /// ```
        pub fn get_from_ext(ext: &str) -> Option<LANG>{
            match ext {
                $(
                    $(
                        stringify!($ext) => Some(LANG::$camel),
                    )*
                )*
                _ => None,
            }
        }
    };
}

macro_rules! mk_emacs_mode {
    ( $( ($camel:ident, [ $( $emacs_mode:expr ),* ]) ),* ) => {
        /// Detects the language associated to the input `Emacs` mode.
        ///
        /// An `Emacs` mode is used to detect a language according to
        /// particular text-information contained in a file.
        ///
        /// # Examples
        ///
        /// ```
        /// use rust_code_analysis::get_from_emacs_mode;
        ///
        /// let emacs_mode = "rust";
        ///
        /// get_from_emacs_mode(emacs_mode).unwrap();
        /// ```
        pub fn get_from_emacs_mode(mode: &str) -> Option<LANG>{
            match mode {
                $(
                    $(
                        $emacs_mode => Some(LANG::$camel),
                    )*
                )*
                _ => None,
            }
        }
    };
}

macro_rules! mk_code {
    ( $( ($camel:ident, $code:ident, $parser:ident, $name:ident, $docname:expr) ),* ) => {
        $(
            pub struct $code { _guard: (), }

            impl LanguageInfo for $code {
                type BaseLang = $camel;

                fn get_lang() -> LANG {
                    LANG::$camel
                }

                fn get_lang_name() -> &'static str {
                    $docname
                }
            }

            #[doc = "The `"]
            #[doc = $docname]
            #[doc = "` language parser."]
            pub type $parser = Parser<$code>;
        )*
    };
}

macro_rules! mk_langs {
    ( $( ($camel:ident, $description: expr, $display: expr, $code:ident, $parser:ident, $name:ident, [ $( $ext:ident ),* ], [ $( $emacs_mode:expr ),* ]) ),* ) => {
        mk_lang!($( ($camel, $name, $display, $description) ),*);
        mk_action!($( ($camel, $parser) ),*);
        mk_extensions!($( ($camel, [ $( $ext ),* ]) ),*);
        mk_emacs_mode!($( ($camel, [ $( $emacs_mode ),* ]) ),*);
        mk_code!($( ($camel, $code, $parser, $name, stringify!($camel)) ),*);
    };
}

pub(crate) use implement_metric_trait;
pub(crate) use {
    get_language, mk_action, mk_code, mk_emacs_mode, mk_extensions, mk_lang, mk_langs,
};
