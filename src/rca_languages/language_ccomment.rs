// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq)]
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

impl Into<&'static str> for Ccomment {
    fn into(self) -> &'static str {
        match self {
            Ccomment::End => "end",
            Ccomment::Nothing => "nothing",
            Ccomment::PreprocContinuationLine => "preproc_continuation_line",
            Ccomment::PreprocLine => "preproc_line",
            Ccomment::DefineToken1 => "define_token1",
            Ccomment::StringLiteralToken1 => "string_literal_token1",
            Ccomment::CharLiteralToken1 => "char_literal_token1",
            Ccomment::Comment => "comment",
            Ccomment::RawStringLiteral => "raw_string_literal",
            Ccomment::TranslationUnit => "translation_unit",
            Ccomment::TopLevelItem => "_top_level_item",
            Ccomment::Define => "define",
            Ccomment::StringLiteral => "string_literal",
            Ccomment::CharLiteral => "char_literal",
            Ccomment::TranslationUnitRepeat1 => "translation_unit_repeat1",
            Ccomment::DefineRepeat1 => "define_repeat1",
            Ccomment::Error => "ERROR",
        }
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Ccomment> = ::phf::Map {
    key: 3558916427560184125,
    disps: ::phf::Slice::Static(&[(2, 0), (0, 11), (0, 13), (0, 10)]),
    entries: ::phf::Slice::Static(&[
        ("char_literal_token1", Ccomment::CharLiteralToken1),
        ("define_token1", Ccomment::DefineToken1),
        ("raw_string_literal", Ccomment::RawStringLiteral),
        ("define", Ccomment::Define),
        ("string_literal", Ccomment::StringLiteral),
        ("define_repeat1", Ccomment::DefineRepeat1),
        ("translation_unit", Ccomment::TranslationUnit),
        ("string_literal_token1", Ccomment::StringLiteralToken1),
        ("ERROR", Ccomment::Error),
        ("comment", Ccomment::Comment),
        ("preproc_line", Ccomment::PreprocLine),
        ("_top_level_item", Ccomment::TopLevelItem),
        ("char_literal", Ccomment::CharLiteral),
        ("end", Ccomment::End),
        ("translation_unit_repeat1", Ccomment::TranslationUnitRepeat1),
        ("nothing", Ccomment::Nothing),
        (
            "preproc_continuation_line",
            Ccomment::PreprocContinuationLine,
        ),
    ]),
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
        unsafe { std::mem::transmute(x as u8) }
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
