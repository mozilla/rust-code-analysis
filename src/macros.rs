#[doc(hidden)]
#[macro_export]
macro_rules! mk_checker {
    ( $name:ident, $( $type:ident ),* ) => {
        #[inline(always)]
        #[allow(unused_variables)]
        fn $name(node: &Node) -> bool {
            let typ = node.kind_id();
            false
            $(
                || typ == <Self as TSLanguage>::BaseLang::$type
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! mk_extern {
    ( $( $name:ident ),* ) => {
        $(
            extern "C" { pub(crate) fn $name() -> Language; }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
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

#[doc(hidden)]
#[macro_export]
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
            /// # fn main() {
            /// println!("{}", LANG::Rust.get_name());
            /// # }
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

#[doc(hidden)]
#[macro_export]
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
        /// # fn main() {
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
        ///     output_format: None,
        ///     pretty: false,
        ///     output_path: None,
        /// };
        ///
        /// action::<Metrics>(&language, source_as_vec, &cfg.path.clone(), None, cfg);
        /// # }
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! mk_extensions {
    ( $( ($camel:ident, [ $( $ext:ident ),* ]) ),* ) => {
        /// Detects the language associated to the input file extension.
        ///
        /// # Examples
        ///
        /// ```
        /// use rust_code_analysis::get_from_ext;
        ///
        /// # fn main() {
        /// let ext = "rs";
        ///
        /// get_from_ext(ext).unwrap();
        /// # }
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

#[doc(hidden)]
#[macro_export]
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
        /// # fn main() {
        /// let emacs_mode = "rust";
        ///
        /// get_from_emacs_mode(emacs_mode).unwrap();
        /// # }
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

#[doc(hidden)]
#[macro_export]
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
            pub type $parser = TSParser<$code>;
        )*
    };
}

#[doc(hidden)]
#[macro_export]
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

#[doc(hidden)]
#[macro_export]
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

#[doc(hidden)]
#[macro_export]
macro_rules! check_metrics {
    ($source: expr, $file: expr, $parser: ident, $metric: ident,
     [ $( ( $func: ident, $true_value: expr $(,$type: ty)? )$(,)* )* ]) => {
        {
            let path = PathBuf::from($file);
            let parser = $parser::new($source.to_string().into_bytes(), &path, None);
            let func_space = metrics(&parser, &path).unwrap();

            $( assert_eq!(func_space.metrics.$metric.$func() $(as $type)?, $true_value); )*
        }
    };
}
