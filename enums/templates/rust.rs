// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum {{ c_name }} {
    {% for (name, _, _) in names -%}
    {{ name }} = {{ loop.index0 }},
    {% endfor %}
}

impl std::fmt::Display for {{ c_name }} {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, &'static str> = {{ phf_map }};

impl From<{{ c_name }}> for &str {
    #[inline(always)]
    fn from(grammar: {{ c_name }}) -> &'static str {
        KEYS.get(grammar.to_string().as_str()).unwrap()
    }
}

impl From<u16> for {{ c_name }} {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// {{ c_name }} == u16
impl PartialEq<u16> for {{ c_name }} {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == {{ c_name }}::from(*x)
    }
}

// u16 == {{ c_name }}
impl PartialEq<{{ c_name }}> for u16 {
    #[inline(always)]
    fn eq(&self, x: &{{ c_name }}) -> bool {
        *x == *self
    }
}

