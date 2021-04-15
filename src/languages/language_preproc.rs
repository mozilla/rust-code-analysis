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

impl std::fmt::Display for Preproc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, &'static str> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 0),
        (0, 6),
        (16, 7),
        (1, 18),
        (11, 4),
        (0, 0),
        (0, 3),
        (3, 16),
    ]),
    entries: ::phf::Slice::Static(&[
        ("TopLevelItem", "_top_level_item"),
        ("Error", "ERROR"),
        ("LF", "\n"),
        ("UndefToken1", "undef_token1"),
        ("LT", "<"),
        ("CharLiteralToken1", "char_literal_token1"),
        ("TranslationUnitRepeat1", "translation_unit_repeat1"),
        ("StringLiteralToken1", "string_literal_token1"),
        ("Comment", "comment"),
        ("IntegerLiteral", "integer_literal"),
        ("RawStringLiteral", "raw_string_literal"),
        ("Path", "path"),
        ("PreprocIfRepeat1", "preproc_if_repeat1"),
        ("PreprocIncludeToken1", "preproc_include_token1"),
        ("PreprocElifToken1", "preproc_elif_token1"),
        ("CharLiteral", "char_literal"),
        ("PreprocNothingToken1", "preproc_nothing_token1"),
        ("StringLiteral", "string_literal"),
        ("PreprocNothing", "preproc_nothing"),
        ("PreprocIfToken1", "preproc_if_token1"),
        ("PreprocContinuationLine", "preproc_continuation_line"),
        ("PreprocElse", "preproc_else"),
        ("DefineRepeat1", "define_repeat1"),
        ("PreprocIfToken2", "preproc_if_token2"),
        ("Nothing", "nothing"),
        ("PreprocElif", "preproc_elif"),
        ("PreprocIf", "preproc_if"),
        ("Define", "define"),
        ("DefineToken1", "define_token1"),
        ("PreprocElseToken1", "preproc_else_token1"),
        ("PreprocInclude", "preproc_include"),
        ("GT", ">"),
        ("Undef", "undef"),
        ("PreprocLine", "preproc_line"),
        ("TranslationUnit", "translation_unit"),
        ("Identifier", "identifier"),
        ("End", "end"),
    ]),
};

impl From<Preproc> for &str {
    #[inline(always)]
    fn from(grammar: Preproc) -> &'static str {
        KEYS.get(grammar.to_string().as_str()).unwrap()
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
