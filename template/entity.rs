use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct {{class_name}}Entity{
    {% for c in columns %}
    pub {{c.columnName}}: Option<{{c.javaType}}>,
    {% endfor %}
}