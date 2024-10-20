use rbatis::executor::Executor;
use rbatis::{crud, html_sql};
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::datetime::DateTime;
use crate::entity::gen_table_entity::{GenTableEntity, GenTableColumnEntity};
use crate::model::gen_table_model::{
    GenTableList, DbTableList, GenTableColumnList, GenTableAddPayload, GenTableColumnAddPayload,
    GenTableListAll
};

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_list(rb: &mut dyn Executor) ->rbatis::Result<Vec<GenTableList>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_page(
    rb: &mut dyn Executor,page_num:u64,page_size:u64, table_name:Option<String>,
    table_comment:Option<String>, begin_time:Option<DateTime>
) ->rbatis::Result<Vec<GenTableList>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_count(
    rb: &mut dyn Executor,table_name:Option<String>,
    table_comment:Option<String>,begin_time:Option<DateTime>
) ->rbatis::Result<u64> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_by_id(rb: &mut dyn Executor,table_id:String
) ->rbatis::Result<Vec<GenTableList>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_all(rb: &mut dyn Executor
) ->rbatis::Result<Vec<GenTableListAll>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_column_by_id(
    rb: &mut dyn Executor,table_id:String
) ->rbatis::Result<Vec<GenTableColumnList>> {
    impled!()
}

// /column/{tableId}
// /createTable
// /updateTable

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn del_gen_table_by_id(
    rb: &mut dyn Executor,table_id:String
) ->rbatis::Result<ExecResult> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn del_gen_table_column_by_id(
    rb: &mut dyn Executor,table_id:String
) ->rbatis::Result<ExecResult> {
    impled!()
}

// /preview/{tableId}
// /download/{tableId}
// /genCode/{tableId}
// /syncDb
// /batchGenCode

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_db_table_page(
    rb: &mut dyn Executor,page_num:u64,page_size:u64
) ->rbatis::Result<Vec<DbTableList>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_db_table_count(rb: &mut dyn Executor) -> rbatis::Result<u64> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_db_table_by_names(
    rb: &mut dyn Executor, names: Vec<&str>
) ->rbatis::Result<Vec<GenTableAddPayload>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_column_list_by_ids(
    rb: &mut dyn Executor, table_ids: Vec<String>
) ->rbatis::Result<Vec<GenTableColumnList>> {
    impled!()
}

#[html_sql("src/mapper/xml/gen_xml.html")]
pub async fn get_gen_table_columns_by_name(
    rb: &mut dyn Executor, table_name: String
) ->rbatis::Result<Vec<GenTableColumnAddPayload>> {
    impled!()
}

crud!(GenTableEntity{},"gen_table");
crud!(GenTableColumnEntity{},"gen_table_column");

crud!(GenTableAddPayload{},"gen_table");
crud!(GenTableColumnAddPayload{},"gen_table_column");

crud!(GenTableColumnList{},"gen_table_column");