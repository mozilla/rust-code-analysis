// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq)]
pub enum Html {
    End = 0,
    LTBANG = 1,
    DoctypeToken1 = 2,
    GT = 3,
    Doctype = 4,
    LT = 5,
    SLASHGT = 6,
    LTSLASH = 7,
    EQ = 8,
    AttributeName = 9,
    AttributeValue = 10,
    SQUOTE = 11,
    AttributeValue2 = 12,
    DQUOTE = 13,
    AttributeValue3 = 14,
    Text = 15,
    TagName = 16,
    TagName2 = 17,
    TagName3 = 18,
    TagName4 = 19,
    ErroneousEndTagName = 20,
    ImplicitEndTag = 21,
    RawText = 22,
    Comment = 23,
    Fragment = 24,
    Doctype2 = 25,
    Node = 26,
    Element = 27,
    ScriptElement = 28,
    StyleElement = 29,
    StartTag = 30,
    StartTag2 = 31,
    StartTag3 = 32,
    SelfClosingTag = 33,
    EndTag = 34,
    ErroneousEndTag = 35,
    Attribute = 36,
    QuotedAttributeValue = 37,
    FragmentRepeat1 = 38,
    StartTagRepeat1 = 39,
    Error = 40,
}

impl Into<&'static str> for Html {
    fn into(self) -> &'static str {
        match self {
            Html::End => "end",
            Html::LTBANG => "<!",
            Html::DoctypeToken1 => "doctype_token1",
            Html::GT => ">",
            Html::Doctype => "doctype",
            Html::LT => "<",
            Html::SLASHGT => "/>",
            Html::LTSLASH => "</",
            Html::EQ => "=",
            Html::AttributeName => "attribute_name",
            Html::AttributeValue => "attribute_value",
            Html::SQUOTE => "'",
            Html::AttributeValue2 => "attribute_value",
            Html::DQUOTE => "\"",
            Html::AttributeValue3 => "attribute_value",
            Html::Text => "text",
            Html::TagName => "tag_name",
            Html::TagName2 => "tag_name",
            Html::TagName3 => "tag_name",
            Html::TagName4 => "tag_name",
            Html::ErroneousEndTagName => "erroneous_end_tag_name",
            Html::ImplicitEndTag => "_implicit_end_tag",
            Html::RawText => "raw_text",
            Html::Comment => "comment",
            Html::Fragment => "fragment",
            Html::Doctype2 => "doctype",
            Html::Node => "_node",
            Html::Element => "element",
            Html::ScriptElement => "script_element",
            Html::StyleElement => "style_element",
            Html::StartTag => "start_tag",
            Html::StartTag2 => "start_tag",
            Html::StartTag3 => "start_tag",
            Html::SelfClosingTag => "self_closing_tag",
            Html::EndTag => "end_tag",
            Html::ErroneousEndTag => "erroneous_end_tag",
            Html::Attribute => "attribute",
            Html::QuotedAttributeValue => "quoted_attribute_value",
            Html::FragmentRepeat1 => "fragment_repeat1",
            Html::StartTagRepeat1 => "start_tag_repeat1",
            Html::Error => "ERROR",
        }
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Html> = ::phf::Map {
    key: 732231254413039614,
    disps: ::phf::Slice::Static(&[(0, 0), (1, 0), (0, 1), (2, 3), (7, 16), (0, 17), (26, 0)]),
    entries: ::phf::Slice::Static(&[
        ("fragment", Html::Fragment),
        ("doctype_token1", Html::DoctypeToken1),
        ("<", Html::LT),
        ("script_element", Html::ScriptElement),
        ("text", Html::Text),
        ("erroneous_end_tag_name", Html::ErroneousEndTagName),
        ("attribute", Html::Attribute),
        ("start_tag_repeat1", Html::StartTagRepeat1),
        ("comment", Html::Comment),
        ("end_tag", Html::EndTag),
        ("attribute_value", Html::AttributeValue),
        ("\'", Html::SQUOTE),
        ("<!", Html::LTBANG),
        ("self_closing_tag", Html::SelfClosingTag),
        ("ERROR", Html::Error),
        ("attribute_name", Html::AttributeName),
        ("quoted_attribute_value", Html::QuotedAttributeValue),
        ("tag_name", Html::TagName),
        ("=", Html::EQ),
        ("start_tag", Html::StartTag),
        ("_implicit_end_tag", Html::ImplicitEndTag),
        (">", Html::GT),
        ("end", Html::End),
        ("style_element", Html::StyleElement),
        ("/>", Html::SLASHGT),
        ("</", Html::LTSLASH),
        ("\\\"", Html::DQUOTE),
        ("erroneous_end_tag", Html::ErroneousEndTag),
        ("fragment_repeat1", Html::FragmentRepeat1),
        ("element", Html::Element),
        ("doctype", Html::Doctype),
        ("_node", Html::Node),
        ("raw_text", Html::RawText),
    ]),
};

impl From<&str> for Html {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
    }
}

impl From<u16> for Html {
    #[inline(always)]
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x as u8) }
    }
}

// Html == u16
impl PartialEq<u16> for Html {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Html::from(*x)
    }
}

// u16 == Html
impl PartialEq<Html> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Html) -> bool {
        *x == *self
    }
}
