// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

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

impl From<Ccomment> for &'static str {
    #[inline(always)]
    fn from(tok: Ccomment) -> Self {
        match tok {
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
