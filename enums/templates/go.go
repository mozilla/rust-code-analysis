// Code generated; DO NOT EDIT.

package {{ c_name.to_lowercase() }}

type SyntaxType{{ c_name }} int16

const (
    {% for (_, _, _, name) in names -%}
    {{ name }} SyntaxType{{ c_name }} = iota
    {% endfor %}
)

// String return the string version of the type
func (st SyntaxType{{ c_name }}) String() string {
	switch st {
		{% for (name, _, ts_name, _) in names -%}
	case {{ name }}:
		return "{{ ts_name }}";
    {% endfor %}
	}
	panic("Unsupported SyntaxType{{ c_name }}")
}

// FromString a SyntaxType{{ c_name }} from the string, panic if not found
func FromString(str String) SyntaxType{{ c_name }} {
	switch str {
		{% for (name, dup, ts_name, _) in names -%}
		{% if !dup %}
	case "{{ ts_name }}":
		return {{ name }};
		{%- endif -%}
		{% endfor %}
	}
	panic("Unsupported SyntaxType{{ c_name }}")
}
