use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use crate::entity::gen_table_entity::GenTableEntity;
use crate::GLOBAL_DB;
use crate::mapper::gen_table_mapper;
use crate::model::common_model::Page;
use crate::model::gen_table_model::{DbTableList, GenTableList};
use crate::utils::func::{create_page, create_page_list, is_modify_ok};
use crate::utils::common;

// query table "gen_table"
// return: list "gen_table" rows of page "page_num"
pub async fn get_gen_table_page(
    page_num:u64,page_size:u64,table_name:Option<String>,
    table_comment:Option<String>,begin_time:Option<DateTime>
) ->rbatis::Result<Page<GenTableList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = gen_table_mapper::get_gen_table_page(
        &mut GLOBAL_DB.clone(),num,size,table_name.clone(),table_comment.clone(),begin_time.clone()
    ).await?;
    let total = gen_table_mapper::get_gen_table_count(
        &mut GLOBAL_DB.clone(),table_name,table_comment,begin_time.clone()
    ).await?;
    Ok(create_page_list(list,total))
}

pub async fn get_gen_table_by_id(id:i64)->rbatis::Result<Option<GenTableList>>{
    let list = gen_table_mapper::get_gen_table_by_id(&mut GLOBAL_DB.clone(),id).await?;
    let one = list.get(0).cloned();
    Ok(one)
}

pub async fn del_gen_table_by_id(table_id:String)->rbatis::Result<bool>{
    let rows = gen_table_mapper::del_gen_table_by_id(&mut GLOBAL_DB.clone(),table_id).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

// query all tables info in database
// return: list tables info of page "page_num"
pub async fn get_db_table_page(page_num:u64,page_size:u64) ->rbatis::Result<Page<DbTableList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = gen_table_mapper::get_db_table_page(
        &mut GLOBAL_DB.clone(),num,size).await?;
    let total = gen_table_mapper::get_db_table_count(&mut GLOBAL_DB.clone()).await?;
    Ok(create_page_list(list,total))
}

// query tables info by table name
// return: tables info vector
pub async fn get_db_table_by_names(names: String) -> rbatis::Result<Vec<DbTableList>> {
    Ok(gen_table_mapper::get_db_table_by_names(&mut GLOBAL_DB.clone(),names).await?)
}

pub async fn import_tables(table_info: Vec<DbTableList>) ->rbatis::Result<bool> {
    let mut rows = ExecResult{rows_affected: 0, last_insert_id: rbs::to_value!(0)};
    for table in table_info {
        let class_name = table.table_name.clone().map(
            |s| common::to_pascal_case(s.as_str())
        );

        let gen_table = GenTableEntity{
            table_id: None,
            table_name: table.table_name.clone(),
            table_comment: table.table_comment.clone(),
            sub_table_name: None,
            sub_table_fk_name: None,
            class_name,
            tpl_category: None,
            package_name: None,
            module_name: None,
            business_name: table.table_name,
            function_name: table.table_comment,
            function_author: None,
            gen_type: None,
            gen_path: None,
            options: None,
            create_by: None,
            create_time: table.create_time,
            update_by: None,
            update_time: table.update_time,
            remark: None,
        };
        rows = GenTableEntity::insert(&mut GLOBAL_DB.clone(),&gen_table).await?;
    }
    Ok(is_modify_ok(rows.rows_affected))
}
