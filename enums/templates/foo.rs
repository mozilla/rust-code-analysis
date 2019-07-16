// Generated. DON'T MODIFY BY HAND!

#[derive(Debug, PartialEq)]
pub enum {{ c_name }} {
    {% for (name, _, _) in names %}
    {{ name }} = {{ loop.index }};
    {% endfor %}
}
