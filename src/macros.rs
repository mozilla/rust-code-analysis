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

#[macro_export]
macro_rules! mk_extern {
    ( $( $name:ident ),* ) => {
        $(
            extern "C" { pub fn $name() -> Language; }
        )*
    };
}

#[macro_export]
macro_rules! mk_enum {
    ( $( $camel:ident ),* ) => {
        #[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
        pub enum LANG {
            $(
                $camel,
            )*
        }
    };
}

#[macro_export]
macro_rules! mk_impl_lang {
    ( $( ($camel:ident, $name:ident, $display: expr) ),* ) => {
        impl LANG {

            pub fn get_language(&self) -> Language {
                unsafe {
                    match self {
                        $(
                            LANG::$camel => $name(),
                        )*
                    }
                }
            }

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

#[macro_export]
macro_rules! mk_action {
    ( $( ($camel:ident, $parser:ident) ),* ) => {
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

#[macro_export]
macro_rules! mk_extensions {
    ( $( ($camel:ident, [ $( $ext:ident ),* ]) ),* ) => {
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

#[macro_export]
macro_rules! mk_emacs_mode {
    ( $( ($camel:ident, [ $( $emacs_mode:expr ),* ]) ),* ) => {
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

#[macro_export]
macro_rules! mk_code {
    ( $( ($camel:ident, $code:ident, $parser:ident, $name:ident) ),* ) => {
        $(
            pub struct $code { }

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
            pub type $parser = TSParser<$code>;
        )*
    };
}

#[macro_export]
macro_rules! mk_langs {
    ( $( ($camel:ident, $display: expr, $code:ident, $parser:ident, $name:ident, [ $( $ext:ident ),* ], [ $( $emacs_mode:expr ),* ]) ),* ) => {
        mk_extern!($( $name ),*);
        mk_enum!($( $camel ),*);
        mk_impl_lang!($( ($camel, $name, $display) ),*);
        mk_action!($( ($camel, $parser) ),*);
        mk_extensions!($( ($camel, [ $( $ext ),* ]) ),*);
        mk_emacs_mode!($( ($camel, [ $( $emacs_mode ),* ]) ),*);
        mk_code!($( ($camel, $code, $parser, $name) ),*);
    };
}

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
