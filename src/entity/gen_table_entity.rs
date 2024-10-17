use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GenTableEntity {
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

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GenTableColumnEntity {
    pub column_id: Option<i64>,
    pub table_id: Option<i64>,
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
