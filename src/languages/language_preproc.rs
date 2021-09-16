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
    key: 12913932095322966823,
    disps: &[
        (1, 0),
        (2, 13),
        (0, 28),
        (2, 16),
        (0, 0),
        (0, 27),
        (5, 36),
        (12, 16),
    ],
    entries: &[
        ("preproc_elif_token1", Preproc::PreprocElifToken1),
        ("preproc_else", Preproc::PreprocElse),
        ("preproc_nothing_token1", Preproc::PreprocNothingToken1),
        ("raw_string_literal", Preproc::RawStringLiteral),
        ("_top_level_item", Preproc::TopLevelItem),
        ("end", Preproc::End),
        ("preproc_else_token1", Preproc::PreprocElseToken1),
        ("\\n", Preproc::LF),
        ("preproc_if", Preproc::PreprocIf),
        ("undef", Preproc::Undef),
        ("define_token1", Preproc::DefineToken1),
        ("preproc_line", Preproc::PreprocLine),
        ("string_literal_token1", Preproc::StringLiteralToken1),
        ("path", Preproc::Path),
        ("define_repeat1", Preproc::DefineRepeat1),
        ("translation_unit", Preproc::TranslationUnit),
        ("preproc_if_token1", Preproc::PreprocIfToken1),
        ("char_literal_token1", Preproc::CharLiteralToken1),
        (
            "preproc_continuation_line",
            Preproc::PreprocContinuationLine,
        ),
        ("undef_token1", Preproc::UndefToken1),
        ("define", Preproc::Define),
        ("preproc_nothing", Preproc::PreprocNothing),
        ("char_literal", Preproc::CharLiteral),
        ("preproc_elif", Preproc::PreprocElif),
        ("nothing", Preproc::Nothing),
        ("<", Preproc::LT),
        ("preproc_include", Preproc::PreprocInclude),
        ("translation_unit_repeat1", Preproc::TranslationUnitRepeat1),
        ("string_literal", Preproc::StringLiteral),
        ("identifier", Preproc::Identifier),
        ("preproc_if_token2", Preproc::PreprocIfToken2),
        ("integer_literal", Preproc::IntegerLiteral),
        ("preproc_include_token1", Preproc::PreprocIncludeToken1),
        ("preproc_if_repeat1", Preproc::PreprocIfRepeat1),
        ("ERROR", Preproc::Error),
        ("comment", Preproc::Comment),
        (">", Preproc::GT),
    ],
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
