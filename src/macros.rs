#[macro_use]
macro_rules! mk_checker {
    ( $name:ident, $( $type:ident ),* ) => {
        #[inline(always)]
        #[allow(unused_variables)]
        fn $name(node: &Node) -> bool {
            let typ = node.object().kind_id();
            false
            $(
                || typ == <Self as TSLanguage>::BaseLang::$type
            )*
        }
    };
}

#[macro_use]
macro_rules! mk_else_if {
    ($if_type:ident) => {
        #[inline(always)]
        fn is_else_if(node: &Node) -> bool {
            if node.object().kind_id() != <Self as TSLanguage>::BaseLang::$if_type {
                return false;
            }
            if let Some(parent) = node.object().parent() {
                return node.object().kind_id() == <Self as TSLanguage>::BaseLang::$if_type
                    && parent.kind_id() == <Self as TSLanguage>::BaseLang::$if_type;
            }
            false
        }
    };
}

#[macro_use]
macro_rules! mk_extern {
    ( $( $name:ident ),* ) => {
        $(
            extern "C" { pub(crate) fn $name() -> Language; }
        )*
    };
}

#[macro_use]
macro_rules! mk_enum {
    ( $( $camel:ident, $description:expr ),* ) => {
        /// The list of supported languages.
        #[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
        pub enum LANG {
            $(
                #[doc = $description]
                $camel,
            )*
        }
    };
}

#[macro_use]
macro_rules! mk_impl_lang {
    ( $( ($camel:ident, $name:ident, $display: expr) ),* ) => {
        impl LANG {

            #[allow(dead_code)]
            pub(crate) fn get_language(&self) -> Language {
                unsafe {
                    match self {
                        $(
                            LANG::$camel => $name(),
                        )*
                    }
                }
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
        }
    };
}

#[macro_use]
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
        pub fn action<T: Callback>(lang: &LANG, source: Vec<u8>, path: &PathBuf, pr: Option<Arc<PreprocResults>>, cfg: T::Cfg) -> T::Res {
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
        pub fn get_function_spaces(lang: &LANG, source: Vec<u8>, path: &PathBuf, pr: Option<Arc<PreprocResults>>) -> Option<FuncSpace> {
            match lang {
                $(
                    LANG::$camel => {
                        let parser = $parser::new(source, &path, pr);
                        metrics(&parser, &path)
                    },
                )*
            }
        }
    };
}

#[macro_use]
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

#[macro_use]
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

#[macro_use]
macro_rules! mk_code {
    ( $( ($camel:ident, $code:ident, $parser:ident, $name:ident, $docname:expr) ),* ) => {
        $(
            pub struct $code { _guard: (), }
            impl CodeMetricsT for $code { }

            impl TSLanguage for $code {
                type BaseLang = $camel;

                fn get_lang() -> LANG {
                    LANG::$camel
                }

                fn get_language() -> Language {
                    unsafe { $name() }
                }

                fn get_lang_name() -> &'static str {
                    stringify!($camel)
                }
            }

            #[doc = "The `"]
            #[doc = $docname]
            #[doc = "` language parser."]
            pub type $parser = Parser<$code>;
        )*
    };
}

#[macro_use]
macro_rules! mk_langs {
    ( $( ($camel:ident, $description: expr, $display: expr, $code:ident, $parser:ident, $name:ident, [ $( $ext:ident ),* ], [ $( $emacs_mode:expr ),* ]) ),* ) => {
        mk_extern!($( $name ),*);
        mk_enum!($( $camel, $description ),*);
        mk_impl_lang!($( ($camel, $name, $display) ),*);
        mk_action!($( ($camel, $parser) ),*);
        mk_extensions!($( ($camel, [ $( $ext ),* ]) ),*);
        mk_emacs_mode!($( ($camel, [ $( $emacs_mode ),* ]) ),*);
        mk_code!($( ($camel, $code, $parser, $name, stringify!($camel)) ),*);
    };
}

#[macro_use]
macro_rules! color {
    ( $stdout: ident, $color: ident) => {
        $stdout.set_color(ColorSpec::new().set_fg(Some(Color::$color)))?;
    };
    ( $stdout: ident, $color: ident, $intense: ident) => {
        $stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::$color))
                .set_intense($intense),
        )?;
    };
}

#[macro_use]
macro_rules! check_metrics {
    ($source: expr, $file: expr, $parser: ident, $metric: ident,
     [ $( ( $func_int: ident, $true_int_value: expr $(,$type_int: ty)? )$(,)* )* ]$(,)*
     $( [ $( ( $func_float: ident, $true_float_value: expr )$(,)* )* ] )?) => {
        {
            let path = PathBuf::from($file);
            let mut trimmed_bytes = $source.trim_end().trim_matches('\n').as_bytes().to_vec();
            trimmed_bytes.push(b'\n');
            let parser = $parser::new(trimmed_bytes, &path, None);
            let func_space = metrics(&parser, &path).unwrap();

            $( assert_eq!(func_space.metrics.$metric.$func_int() $(as $type_int)?, $true_int_value); )*

            $(
                $(
                    assert!((func_space.metrics.$metric.$func_float() - $true_float_value).abs() < f64::EPSILON);
                )*
            )?
        }
    };
}
