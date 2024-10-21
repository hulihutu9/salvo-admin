use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use crate::model::gen_table_model::GenTableModifyPayload;

#[derive(Debug,Serialize,Deserialize,Clone, Default)]
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

#[derive(Debug,Serialize,Deserialize,Clone, Default)]
pub struct GenTableColumnEntity {
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

impl From<GenTableModifyPayload> for GenTableEntity {
    fn from(a: GenTableModifyPayload) -> Self {
        GenTableEntity {
            table_id: a.table_id,
            table_name: a.table_name,
            table_comment: a.table_comment,
            sub_table_name: a.sub_table_name,
            sub_table_fk_name: a.sub_table_fk_name,
            class_name: a.class_name,
            tpl_category: a.tpl_category,
            package_name: a.package_name,
            module_name: a.module_name,
            business_name: a.business_name,
            function_name: a.function_name,
            function_author: a.function_author,
            gen_type: a.gen_type,
            gen_path: a.gen_path,
            options: a.options,
            create_by: a.create_by,
            create_time: a.create_time,
            update_by: a.update_by,
            update_time: a.update_time,
            remark: a.remark,
        }
    }
}
