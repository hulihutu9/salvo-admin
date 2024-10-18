use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use crate::entity::gen_table_entity::{GenTableColumnEntity, GenTableEntity};
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::gen_table_mapper;
use crate::model::common_model::Page;
use crate::model::gen_table_model::{DbTableList, GenTableList};
use crate::utils::func::{create_page, create_page_list, is_modify_ok};
use crate::utils::gen_utils;

// query table "gen_table"
// return: list "gen_table" rows of page "page_num"
pub async fn get_gen_table_page(
    page_num:u64,page_size:u64,table_name:Option<String>,
    table_comment:Option<String>,begin_time:Option<DateTime>
) ->rbatis::Result<Page<GenTableList>>{
    let (num,size) = create_page(page_num,page_size);
    let mut list = gen_table_mapper::get_gen_table_page(
        &mut GLOBAL_DB.clone(),num,size,table_name.clone(),table_comment.clone(),begin_time.clone()
    ).await?;

    if !list.is_empty() {
        // get table "gen_column" rows
        let table_ids: Vec<String> = list.iter().map(|row|
            row.table_id.unwrap_or(-1).to_string()).collect();
        let columns = gen_table_mapper::get_gen_table_column_list_by_ids(
            &mut GLOBAL_DB.clone(), table_ids
        ).await?;

        // set rows to GenTableList
        for gen_table in list.iter_mut() {
            let id = gen_table.table_id.map(|v| v.to_string());
            let res = columns.iter().find(|&row|
                row.table_id == id).cloned();
            gen_table.columns = res;
        }
    }

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
    let rows = gen_table_mapper::del_gen_table_by_id(
        &mut GLOBAL_DB.clone(),table_id.clone()).await?;
    gen_table_mapper::del_gen_table_column_by_id(&mut GLOBAL_DB.clone(),table_id).await?;
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
pub async fn get_db_table_by_names(names: String) -> rbatis::Result<Vec<GenTableEntity>> {
    Ok(gen_table_mapper::get_db_table_by_names(&mut GLOBAL_DB.clone(),names).await?)
}

pub async fn import_tables(user_id: i32, table_list: &mut Vec<GenTableEntity>)
    ->rbatis::Result<bool> {
    let mut rows = ExecResult{rows_affected: 0, last_insert_id: rbs::to_value!(0)};
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();

    for table in table_list.iter_mut() {
        table.class_name = table.table_name.clone();
        table.business_name = gen_utils::get_business_name(table.table_name.clone());
        table.function_name = gen_utils::replace_text(table.table_comment.clone());
        table.create_by = Some(user.user_name.clone());

        rows = GenTableEntity::insert(&mut GLOBAL_DB.clone(), table).await?;

        // insert table "gen_table_column"
        if rows.rows_affected > 0 {
            table.table_id = Some(rbs::from_value(rows.last_insert_id)?);
            let mut gen_table_columns = gen_table_mapper::get_gen_table_columns_by_name(
                &mut GLOBAL_DB.clone(), table.table_name.clone().unwrap()).await?;
            for column in gen_table_columns.iter_mut() {
                gen_utils::init_column_field(column, &table);
                rows = GenTableColumnEntity::insert(&mut GLOBAL_DB.clone(),&column).await?;
            }
        }
    }

    Ok(is_modify_ok(rows.rows_affected))
}
