// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum {{ c_name }} {
    {% for (name, _, _) in names -%}
    {{ name }} = {{ loop.index0 }},
    {% endfor %}
}

impl From<{{ c_name }}> for &'static str {
    #[inline(always)]
    fn from(tok: {{ c_name }}) -> Self {
        match tok {
            {% for (name, _, ts_name) in names -%}
            {{ c_name }}::{{ name }} => "{{ ts_name }}",
            {% endfor %}
        }
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
        *self == Into::<Self>::into(*x)
    }
}

// u16 == {{ c_name }}
impl PartialEq<{{ c_name }}> for u16 {
    #[inline(always)]
    fn eq(&self, x: &{{ c_name }}) -> bool {
        *x == *self
    }
}

