use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use salvo::oapi::{ToParameters, ToSchema};

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct {{class_name}}List{
    {% for c in columns %}
    {% if c.isList %}
    pub {{c.columnName}}: Option<{{c.javaType}}>,
    {% endif %}
    {% endfor %}
}

#[derive(Debug,Serialize,Deserialize,ToParameters,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(default_parameter_in = Query))]
pub struct {{class_name}}ListPayload{
    pub page_num:u64,
    pub page_size:u64,
    {% for c in columns %}
    {% if c.isList %}
    pub {{c.columnName}}: Option<{{c.javaType}}>,
    {% endif %}
    {% endfor %}
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct {{class_name}}ModifyPayload{
    {% for c in columns %}
    {% if c.isEdit %}
    pub {{c.columnName}}: Option<{{c.javaType}}>,
    {% endif %}
    {% endfor %}
}
