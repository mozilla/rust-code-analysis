// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum Ccomment {
    End = 0,
    Nothing = 1,
    PreprocContinuationLine = 2,
    PreprocLine = 3,
    DefineToken1 = 4,
    StringLiteralToken1 = 5,
    CharLiteralToken1 = 6,
    Comment = 7,
    RawStringLiteral = 8,
    TranslationUnit = 9,
    TopLevelItem = 10,
    Define = 11,
    StringLiteral = 12,
    CharLiteral = 13,
    TranslationUnitRepeat1 = 14,
    DefineRepeat1 = 15,
    Error = 16,
}

impl std::fmt::Display for Ccomment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, &'static str> = ::phf::Map {
    key: 3347381344252206323,
    disps: ::phf::Slice::Static(&[(2, 0), (8, 9), (1, 8), (1, 16)]),
    entries: ::phf::Slice::Static(&[
        ("Nothing", "nothing"),
        ("RawStringLiteral", "raw_string_literal"),
        ("StringLiteralToken1", "string_literal_token1"),
        ("DefineToken1", "define_token1"),
        ("CharLiteralToken1", "char_literal_token1"),
        ("CharLiteral", "char_literal"),
        ("Define", "define"),
        ("TranslationUnitRepeat1", "translation_unit_repeat1"),
        ("PreprocLine", "preproc_line"),
        ("TranslationUnit", "translation_unit"),
        ("Error", "ERROR"),
        ("Comment", "comment"),
        ("End", "end"),
        ("StringLiteral", "string_literal"),
        ("TopLevelItem", "_top_level_item"),
        ("PreprocContinuationLine", "preproc_continuation_line"),
        ("DefineRepeat1", "define_repeat1"),
    ]),
};

impl From<Ccomment> for &str {
    #[inline(always)]
    fn from(grammar: Ccomment) -> &'static str {
        KEYS.get(grammar.to_string().as_str()).unwrap()
    }
}

impl From<u16> for Ccomment {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Ccomment == u16
impl PartialEq<u16> for Ccomment {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Ccomment::from(*x)
    }
}

// u16 == Ccomment
impl PartialEq<Ccomment> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Ccomment) -> bool {
        *x == *self
    }
}
