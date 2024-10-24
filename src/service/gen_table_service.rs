use std::collections::BTreeMap;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use crate::entity::gen_table_entity::{GenTableEntity, GenTableColumnEntity};
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::gen_table_mapper;
use crate::model::common_model::Page;
use crate::model::gen_table_model::{
    GenTableList, GenTableModifyPayload, TableInfo
};
use crate::utils::func::{create_page, create_page_list, is_modify_ok};
use crate::utils::gen_utils;
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

pub async fn get_gen_table_by_id(id:String)->rbatis::Result<Option<TableInfo>>{
    let table = gen_table_mapper::get_gen_table_by_id(
        &mut GLOBAL_DB.clone(),id.clone()).await?;
    let table_one = table.get(0).cloned();
    let tables = gen_table_mapper::get_gen_table_all(
        &mut GLOBAL_DB.clone()).await?;
    let list = gen_table_mapper::get_gen_table_column_by_id(
        &mut GLOBAL_DB.clone(),id).await?;

    let mut  res = None;
    if table_one.is_some() && list.len() > 0 {
        res = Some(TableInfo {
            info: table_one,
            rows: Some(list),
            tables: Some(tables),
        });
    }
    Ok(res)
}

pub async fn put_edit_gen_table(user_id: i32, table: GenTableModifyPayload)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(
        &mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let columns = table.columns.clone().unwrap();
    let mut gen_table: GenTableEntity = table.into();
    gen_table.update_by = Some(user.user_name.clone());
    gen_table.update_time = Some(DateTime::now());

    // update gen_table
    let rows = GenTableEntity::update_by_column(
        &mut GLOBAL_DB.clone(),&gen_table,"table_id").await?;

    // update gen_table_column
    if rows.rows_affected.clone() > 0 {
        for column in columns.iter() {
            GenTableColumnEntity::update_by_column(
                &mut GLOBAL_DB.clone(),column,"column_id").await?;
        }
    }
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn del_gen_table_by_id(table_id:String)->rbatis::Result<bool>{
    let rows = gen_table_mapper::del_gen_table_by_id(
        &mut GLOBAL_DB.clone(),table_id.clone()).await?;
    gen_table_mapper::del_gen_table_column_by_id(&mut GLOBAL_DB.clone(),table_id).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

// query all tables info in database
// return: list tables info of page "page_num"
pub async fn get_db_table_page(page_num:u64,page_size:u64) ->rbatis::Result<Page<GenTableList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = gen_table_mapper::get_db_table_page(
        &mut GLOBAL_DB.clone(),num,size).await?;
    let total = gen_table_mapper::get_db_table_count(&mut GLOBAL_DB.clone()).await?;
    Ok(create_page_list(list,total))
}

// query tables info by table name
// return: tables info vector
pub async fn get_db_table_by_names(names: Vec<&str>) -> rbatis::Result<Vec<GenTableEntity>> {
    Ok(gen_table_mapper::get_db_table_by_names(&mut GLOBAL_DB.clone(),names).await?)
}

pub async fn import_tables(user_id: i32, table_names: Vec<&str>)
    ->rbatis::Result<bool> {
    let mut rows = ExecResult{rows_affected: 0, last_insert_id: rbs::to_value!(0)};
    let user = SysUser::select_by_column(
        &mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();

    // Query tables info in database
    let mut table_list = get_db_table_by_names(table_names)
        .await?;

    for table in table_list.iter_mut() {
        table.class_name = Some(common::to_pascal_case(&table.table_name.clone().unwrap()));
        table.business_name = gen_utils::get_business_name(table.table_name.clone());
        table.function_name = gen_utils::replace_text(table.table_comment.clone());
        table.create_by = Some(user.user_name.clone());
        table.function_author = Some(user.user_name.clone());
        table.package_name = Some("other".to_string());
        table.module_name = Some("other".to_string());
        rows = GenTableEntity::insert(&mut GLOBAL_DB.clone(), &table).await?;

        // insert table "gen_table_column"
        if rows.rows_affected > 0 {
            table.table_id = Some(rbs::from_value(rows.last_insert_id)?);
            let mut db_table_columns = gen_table_mapper::get_db_table_columns_by_name(
                &mut GLOBAL_DB.clone(), table.table_name.clone().unwrap()).await?;
            for column in db_table_columns.iter_mut() {
                gen_utils::init_column_field(column, table);
                rows = GenTableColumnEntity::insert(&mut GLOBAL_DB.clone(), &column).await?;
            }
        }
    }

    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_preview_code(id:String)->rbatis::Result<Option<BTreeMap<String,String>>>{
    // set template context
    let context = gen_utils::init_context(id);

    // render template
    let templates = gen_utils::get_template_list();
    let res = gen_utils::render_template(context.await, templates);

    Ok(Some(res))
}
