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

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Ccomment> = ::phf::Map {
    key: 15467950696543387533,
    disps: &[(1, 0), (0, 13), (5, 8), (7, 15)],
    entries: &[
        ("define_token1", Ccomment::DefineToken1),
        ("nothing", Ccomment::Nothing),
        ("char_literal", Ccomment::CharLiteral),
        ("translation_unit", Ccomment::TranslationUnit),
        ("define", Ccomment::Define),
        (
            "preproc_continuation_line",
            Ccomment::PreprocContinuationLine,
        ),
        ("string_literal_token1", Ccomment::StringLiteralToken1),
        ("ERROR", Ccomment::Error),
        ("_top_level_item", Ccomment::TopLevelItem),
        ("char_literal_token1", Ccomment::CharLiteralToken1),
        ("comment", Ccomment::Comment),
        ("translation_unit_repeat1", Ccomment::TranslationUnitRepeat1),
        ("preproc_line", Ccomment::PreprocLine),
        ("string_literal", Ccomment::StringLiteral),
        ("define_repeat1", Ccomment::DefineRepeat1),
        ("raw_string_literal", Ccomment::RawStringLiteral),
        ("end", Ccomment::End),
    ],
};

impl From<&str> for Ccomment {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
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
