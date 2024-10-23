use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use salvo::oapi::{ToParameters, ToSchema};
use crate::entity::gen_table_entity::GenTableColumnEntity;

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct GenTableList {
    pub table_id:Option<i64>,
    pub table_name:Option<String>,
    pub table_comment:Option<String>,
    pub sub_table_name:Option<String>,
    pub sub_table_fk_name:Option<String>,
    pub class_name:Option<String>,
    pub tpl_category:Option<String>,
    pub package_name:Option<String>,
    pub module_name:Option<String>,
    pub business_name:Option<String>,
    pub function_name:Option<String>,
    pub function_author:Option<String>,
    pub gen_type:Option<String>,
    pub gen_path:Option<String>,
    pub options:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
    pub remark:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct GenTableColumnList {
    pub column_id: Option<i64>,
    pub table_id: Option<String>,
    pub column_name: Option<String>,
    pub column_comment: Option<String>,
    pub column_type: Option<String>,
    pub java_type: Option<String>,
    pub java_field: Option<String>,
    pub is_pk: Option<String>,
    pub is_increment: Option<String>,
    pub is_required: Option<String>,
    pub is_insert: Option<String>,
    pub is_edit: Option<String>,
    pub is_list: Option<String>,
    pub is_query: Option<String>,
    pub query_type: Option<String>,
    pub html_type: Option<String>,
    pub dict_type: Option<String>,
    pub sort: Option<i32>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
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
    pub table_name:Option<String>,
    pub table_comment:Option<String>,
    pub sub_table_name:Option<String>,
    pub sub_table_fk_name:Option<String>,
    pub class_name:Option<String>,
    pub tpl_category:Option<String>,
    pub package_name:Option<String>,
    pub module_name:Option<String>,
    pub business_name:Option<String>,
    pub function_name:Option<String>,
    pub function_author:Option<String>,
    pub gen_type:Option<String>,
    pub gen_path:Option<String>,
    pub options:Option<String>,
    pub create_by:Option<String>,
    pub create_time:Option<DateTime>,
    pub update_by:Option<String>,
    pub update_time:Option<DateTime>,
    pub remark:Option<String>,
    pub columns:Option<Vec<GenTableColumnEntity>>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct TableInfo {
    pub info: Option<GenTableList>,
    pub rows: Option<Vec<GenTableColumnList>>,
    pub tables: Option<Vec<GenTableList>>,
}

impl From<GenTableColumnEntity> for GenTableColumnList {
    fn from(a: GenTableColumnEntity) -> Self {
        GenTableColumnList {
            column_id: a.column_id,
            table_id: a.table_id,
            column_name: a.column_name,
            column_comment: a.column_comment,
            column_type: a.column_type,
            java_type: a.java_type,
            java_field: a.java_field,
            is_pk: a.is_pk,
            is_increment: a.is_increment,
            is_required: a.is_required,
            is_insert: a.is_insert,
            is_edit: a.is_edit,
            is_list: a.is_list,
            is_query: a.is_query,
            query_type: a.query_type,
            html_type: a.html_type,
            dict_type: a.dict_type,
            sort: a.sort,
            create_by: a.create_by,
            create_time: a.create_time,
            update_by: a.update_by,
            update_time: a.update_time,
        }
    }
}

