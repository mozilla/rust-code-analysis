// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum {{ c_name }} {
    {% for (name, _, _) in names -%}
    {{ name }} = {{ loop.index0 }},
    {% endfor %}
}

impl Into<&'static str> for {{ c_name }} {
    fn into(self) -> &'static str {
        match self {
            {% for (name, _, ts_name) in names -%}
            {{ c_name }}::{{ name }} => "{{ ts_name }}",
            {% endfor -%}
        }
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, {{ c_name }}> = {{ phf_map }};

impl From<&str> for {{ c_name }} {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
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

