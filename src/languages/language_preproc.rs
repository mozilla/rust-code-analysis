// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
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

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Preproc> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 0),
        (33, 26),
        (3, 12),
        (18, 27),
        (0, 1),
        (3, 27),
        (1, 25),
        (2, 7),
    ]),
    entries: ::phf::Slice::Static(&[
        ("preproc_if_token2", Preproc::PreprocIfToken2),
        ("preproc_elif", Preproc::PreprocElif),
        ("translation_unit_repeat1", Preproc::TranslationUnitRepeat1),
        ("identifier", Preproc::Identifier),
        ("end", Preproc::End),
        ("raw_string_literal", Preproc::RawStringLiteral),
        ("translation_unit", Preproc::TranslationUnit),
        ("preproc_include", Preproc::PreprocInclude),
        ("_top_level_item", Preproc::TopLevelItem),
        ("define", Preproc::Define),
        ("define_token1", Preproc::DefineToken1),
        ("ERROR", Preproc::Error),
        ("preproc_include_token1", Preproc::PreprocIncludeToken1),
        ("preproc_line", Preproc::PreprocLine),
        ("preproc_if", Preproc::PreprocIf),
        ("string_literal_token1", Preproc::StringLiteralToken1),
        ("undef", Preproc::Undef),
        (
            "preproc_continuation_line",
            Preproc::PreprocContinuationLine,
        ),
        ("preproc_else", Preproc::PreprocElse),
        (">", Preproc::GT),
        ("define_repeat1", Preproc::DefineRepeat1),
        ("<", Preproc::LT),
        ("comment", Preproc::Comment),
        ("preproc_nothing", Preproc::PreprocNothing),
        ("preproc_if_token1", Preproc::PreprocIfToken1),
        ("nothing", Preproc::Nothing),
        ("preproc_nothing_token1", Preproc::PreprocNothingToken1),
        ("char_literal", Preproc::CharLiteral),
        ("string_literal", Preproc::StringLiteral),
        ("undef_token1", Preproc::UndefToken1),
        ("preproc_else_token1", Preproc::PreprocElseToken1),
        ("path", Preproc::Path),
        ("preproc_if_repeat1", Preproc::PreprocIfRepeat1),
        ("char_literal_token1", Preproc::CharLiteralToken1),
        ("integer_literal", Preproc::IntegerLiteral),
        ("\\n", Preproc::LF),
        ("preproc_elif_token1", Preproc::PreprocElifToken1),
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
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
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
