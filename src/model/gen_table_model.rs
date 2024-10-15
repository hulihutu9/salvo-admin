use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use salvo::oapi::{ToParameters, ToSchema};

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct GenTableList {
    pub table_id:Option<i64>,
    pub table_name:Option<String>,
    pub table_comment:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_time:Option<DateTime>,
}

#[derive(Debug,Serialize,Deserialize,ToParameters,Clone)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(default_parameter_in = Query))]
pub struct GenTableListPayload {
    pub page_num:u64,
    pub page_size:u64,
    pub table_name:Option<String>,
    pub table_comment:Option<String>,
    pub create_time:Option<DateTime>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct GenTableModifyPayload {
    pub table_id:Option<i64>,
    pub table_title:Option<String>,
    pub table_comment:Option<String>,
    pub class_name:Option<String>,
    pub function_author:Option<String>,
    pub remark:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase", deserialize ="SCREAMING_SNAKE_CASE"))]
pub struct DbTableList {
    pub table_name:Option<String>,
    pub table_comment:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_time:Option<DateTime>,
}
