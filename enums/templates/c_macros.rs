// Code generated; DO NOT EDIT.

const {{ u_name }}: &[&str] = &[
    {% for name in names -%}
    "{{ name }}",
    {% endfor %}
];

pub fn is_{{ l_name }}(mac: &str) -> bool {
   {{ u_name }}.contains(&mac)
 }
