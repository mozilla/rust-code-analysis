// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq)]
pub enum Preproc {
    End = 0,
    Identifier = 1,
    Nothing = 2,
    PreprocContinuationLine = 3,
    PreprocLine = 4,
    PreprocIncludeToken1 = 5,
    LT = 6,
    GT = 7,
    Path = 8,
    DefineToken1 = 9,
    LF = 10,
    PreprocIfToken1 = 11,
    PreprocIfToken2 = 12,
    PreprocElifToken1 = 13,
    PreprocElseToken1 = 14,
    UndefToken1 = 15,
    PreprocNothingToken1 = 16,
    StringLiteralToken1 = 17,
    CharLiteralToken1 = 18,
    IntegerLiteral = 19,
    Comment = 20,
    RawStringLiteral = 21,
    TranslationUnit = 22,
    TopLevelItem = 23,
    PreprocInclude = 24,
    Define = 25,
    PreprocIf = 26,
    PreprocElif = 27,
    PreprocElse = 28,
    Undef = 29,
    PreprocNothing = 30,
    StringLiteral = 31,
    CharLiteral = 32,
    TranslationUnitRepeat1 = 33,
    DefineRepeat1 = 34,
    PreprocIfRepeat1 = 35,
    Error = 36,
    
}

impl Into<&'static str> for Preproc {
    fn into(self) -> &'static str {
        match self {
            Preproc::End => "end",
            Preproc::Identifier => "identifier",
            Preproc::Nothing => "nothing",
            Preproc::PreprocContinuationLine => "preproc_continuation_line",
            Preproc::PreprocLine => "preproc_line",
            Preproc::PreprocIncludeToken1 => "preproc_include_token1",
            Preproc::LT => "<",
            Preproc::GT => ">",
            Preproc::Path => "path",
            Preproc::DefineToken1 => "define_token1",
            Preproc::LF => "\n",
            Preproc::PreprocIfToken1 => "preproc_if_token1",
            Preproc::PreprocIfToken2 => "preproc_if_token2",
            Preproc::PreprocElifToken1 => "preproc_elif_token1",
            Preproc::PreprocElseToken1 => "preproc_else_token1",
            Preproc::UndefToken1 => "undef_token1",
            Preproc::PreprocNothingToken1 => "preproc_nothing_token1",
            Preproc::StringLiteralToken1 => "string_literal_token1",
            Preproc::CharLiteralToken1 => "char_literal_token1",
            Preproc::IntegerLiteral => "integer_literal",
            Preproc::Comment => "comment",
            Preproc::RawStringLiteral => "raw_string_literal",
            Preproc::TranslationUnit => "translation_unit",
            Preproc::TopLevelItem => "_top_level_item",
            Preproc::PreprocInclude => "preproc_include",
            Preproc::Define => "define",
            Preproc::PreprocIf => "preproc_if",
            Preproc::PreprocElif => "preproc_elif",
            Preproc::PreprocElse => "preproc_else",
            Preproc::Undef => "undef",
            Preproc::PreprocNothing => "preproc_nothing",
            Preproc::StringLiteral => "string_literal",
            Preproc::CharLiteral => "char_literal",
            Preproc::TranslationUnitRepeat1 => "translation_unit_repeat1",
            Preproc::DefineRepeat1 => "define_repeat1",
            Preproc::PreprocIfRepeat1 => "preproc_if_repeat1",
            Preproc::Error => "ERROR",
            }
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Preproc> = ::phf::Map {
    key: 3347381344252206323,
    disps: ::phf::Slice::Static(&[
        (12, 20),
        (23, 13),
        (0, 0),
        (0, 2),
        (2, 12),
        (0, 1),
        (2, 30),
        (33, 34),
    ]),
    entries: ::phf::Slice::Static(&[
        ("translation_unit_repeat1", Preproc::TranslationUnitRepeat1),
        ("string_literal", Preproc::StringLiteral),
        ("preproc_if_repeat1", Preproc::PreprocIfRepeat1),
        ("preproc_else_token1", Preproc::PreprocElseToken1),
        ("define_token1", Preproc::DefineToken1),
        ("nothing", Preproc::Nothing),
        ("preproc_nothing_token1", Preproc::PreprocNothingToken1),
        ("preproc_continuation_line", Preproc::PreprocContinuationLine),
        ("path", Preproc::Path),
        ("ERROR", Preproc::Error),
        ("preproc_line", Preproc::PreprocLine),
        ("raw_string_literal", Preproc::RawStringLiteral),
        ("preproc_include", Preproc::PreprocInclude),
        ("identifier", Preproc::Identifier),
        ("undef_token1", Preproc::UndefToken1),
        ("comment", Preproc::Comment),
        ("preproc_include_token1", Preproc::PreprocIncludeToken1),
        ("_top_level_item", Preproc::TopLevelItem),
        ("preproc_if", Preproc::PreprocIf),
        ("preproc_nothing", Preproc::PreprocNothing),
        ("translation_unit", Preproc::TranslationUnit),
        ("\\n", Preproc::LF),
        ("integer_literal", Preproc::IntegerLiteral),
        ("preproc_elif", Preproc::PreprocElif),
        ("char_literal_token1", Preproc::CharLiteralToken1),
        ("undef", Preproc::Undef),
        ("char_literal", Preproc::CharLiteral),
        (">", Preproc::GT),
        ("string_literal_token1", Preproc::StringLiteralToken1),
        ("preproc_if_token2", Preproc::PreprocIfToken2),
        ("define_repeat1", Preproc::DefineRepeat1),
        ("preproc_elif_token1", Preproc::PreprocElifToken1),
        ("end", Preproc::End),
        ("preproc_else", Preproc::PreprocElse),
        ("<", Preproc::LT),
        ("preproc_if_token1", Preproc::PreprocIfToken1),
        ("define", Preproc::Define),
    ]),
};

impl From<&str> for Preproc {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
    }
}

impl From<u16> for Preproc {
    #[inline(always)]
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x as u8) }
    }
}

// Preproc == u16
impl PartialEq<u16> for Preproc {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Preproc::from(*x)
    }
}

// u16 == Preproc
impl PartialEq<Preproc> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Preproc) -> bool {
        *x == *self
    }
}