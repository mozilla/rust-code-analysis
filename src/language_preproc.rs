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
    PreprocNothingToken1 = 15,
    StringLiteralToken1 = 16,
    CharLiteralToken1 = 17,
    IntegerLiteral = 18,
    Comment = 19,
    RawStringLiteral = 20,
    TranslationUnit = 21,
    TopLevelItem = 22,
    PreprocInclude = 23,
    Define = 24,
    PreprocIf = 25,
    PreprocElif = 26,
    PreprocElse = 27,
    PreprocNothing = 28,
    StringLiteral = 29,
    CharLiteral = 30,
    TranslationUnitRepeat1 = 31,
    DefineRepeat1 = 32,
    PreprocIfRepeat1 = 33,
    Error = 34,
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

static KEYS: phf::Map<&'static str, Preproc> = ::phf::Map {
    key: 2575009635930530140,
    disps: ::phf::Slice::Static(&[(1, 0), (3, 0), (0, 19), (1, 1), (1, 1), (9, 1), (19, 28)]),
    entries: ::phf::Slice::Static(&[
        ("path", Preproc::Path),
        ("preproc_if_repeat1", Preproc::PreprocIfRepeat1),
        ("preproc_include", Preproc::PreprocInclude),
        ("end", Preproc::End),
        (
            "preproc_continuation_line",
            Preproc::PreprocContinuationLine,
        ),
        ("preproc_if_token1", Preproc::PreprocIfToken1),
        ("define_repeat1", Preproc::DefineRepeat1),
        ("_top_level_item", Preproc::TopLevelItem),
        ("preproc_elif", Preproc::PreprocElif),
        ("string_literal_token1", Preproc::StringLiteralToken1),
        ("char_literal", Preproc::CharLiteral),
        ("\\n", Preproc::LF),
        ("preproc_if_token2", Preproc::PreprocIfToken2),
        ("preproc_if", Preproc::PreprocIf),
        ("<", Preproc::LT),
        ("preproc_line", Preproc::PreprocLine),
        ("define", Preproc::Define),
        ("char_literal_token1", Preproc::CharLiteralToken1),
        ("preproc_nothing_token1", Preproc::PreprocNothingToken1),
        ("preproc_else_token1", Preproc::PreprocElseToken1),
        (">", Preproc::GT),
        ("raw_string_literal", Preproc::RawStringLiteral),
        ("preproc_elif_token1", Preproc::PreprocElifToken1),
        ("ERROR", Preproc::Error),
        ("nothing", Preproc::Nothing),
        ("preproc_else", Preproc::PreprocElse),
        ("string_literal", Preproc::StringLiteral),
        ("identifier", Preproc::Identifier),
        ("preproc_nothing", Preproc::PreprocNothing),
        ("define_token1", Preproc::DefineToken1),
        ("translation_unit_repeat1", Preproc::TranslationUnitRepeat1),
        ("translation_unit", Preproc::TranslationUnit),
        ("comment", Preproc::Comment),
        ("preproc_include_token1", Preproc::PreprocIncludeToken1),
        ("integer_literal", Preproc::IntegerLiteral),
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
