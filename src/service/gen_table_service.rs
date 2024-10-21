use std::collections::HashMap;
use handlebars::to_json;
use serde_json::value::Map;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use crate::entity::gen_table_entity::{GenTableEntity, GenTableColumnEntity};
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::gen_table_mapper;
use crate::model::common_model::Page;
use crate::model::gen_table_model::{
    GenTableList, GenTableColumnList, GenTableModifyPayload, TableInfo
};
use crate::service::gen_table_service;
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
            GenTableColumnList::update_by_column(
                &mut GLOBAL_DB.clone(),&column,"column_id").await?;
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
pub async fn get_db_table_by_names(names: Vec<&str>) -> rbatis::Result<Vec<GenTableList>> {
    Ok(gen_table_mapper::get_db_table_by_names(&mut GLOBAL_DB.clone(),names).await?)
}

pub async fn import_tables(user_id: i32, table_names: Vec<&str>)
    ->rbatis::Result<bool> {
    let mut rows = ExecResult{rows_affected: 0, last_insert_id: rbs::to_value!(0)};
    let user = SysUser::select_by_column(
        &mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();

    // Query tables info in database
    let mut table_list = gen_table_service::get_db_table_by_names(table_names)
        .await?;

    for table_info in table_list.iter_mut() {
        let mut table : GenTableEntity = Default::default();
        table.class_name = table_info.table_name.clone();
        table.business_name = gen_utils::get_business_name(table_info.table_name.clone());
        table.function_name = gen_utils::replace_text(table_info.table_comment.clone());
        table.create_by = Some(user.user_name.clone());

        rows = GenTableEntity::insert(&mut GLOBAL_DB.clone(), &table).await?;

        // insert table "gen_table_column"
        if rows.rows_affected > 0 {
            table.table_id = Some(rbs::from_value(rows.last_insert_id)?);
            let mut db_table_columns = gen_table_mapper::get_db_table_columns_by_name(
                &mut GLOBAL_DB.clone(), table.table_name.clone().unwrap()).await?;
            for column_info in db_table_columns.iter_mut() {
                let column = gen_utils::init_column_field(column_info, &table);
                rows = GenTableColumnEntity::insert(&mut GLOBAL_DB.clone(), &column).await?;
            }
        }
    }

    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_preview_code(id:String)->rbatis::Result<Option<HashMap<String,String>>>{
    let tables = gen_table_mapper::get_gen_table_by_id(
        &mut GLOBAL_DB.clone(),id.clone()).await?;
    let table = tables.get(0).unwrap();

    // get sub table info
    let sub_table_name = table.table_name.clone();
    let mut sub_table: Option<GenTableList> = None;
    if let Some(sub_table_name) = sub_table_name {
        let sub_tables = gen_table_mapper::get_db_table_by_names(
            &mut GLOBAL_DB.clone(),vec![&sub_table_name]).await?;
        sub_table = sub_tables.get(0).cloned();
    }

    // get primary key column info
    let mut pk_column: Option<&GenTableColumnList> = None;
    let list = gen_table_mapper::get_gen_table_column_by_id(
        &mut GLOBAL_DB.clone(),id).await?;
    for column in list.iter() {
        if column.is_pk == Some("1".to_string()) {
            pk_column = Some(column);
            break;
        }
    }

    // set template context
    let mut context = Map::new();
    context.insert("world".to_string(), to_json("世界!"));
    context.insert("table_name".to_string(), to_json(table.table_name.clone().unwrap()));
    context.insert("class_name".to_string(), to_json(table.class_name.clone().unwrap()));
    context.insert("module_name".to_string(), to_json(&table.module_name.clone().unwrap()));
    context.insert("business_name".to_string(), to_json(table.business_name.clone().unwrap()));
    context.insert("function_name".to_string(), to_json(table.function_name.clone().unwrap()));

    // render template
    let templates = gen_utils::get_template_list();
    let res = gen_utils::render_template(context, templates);

    Ok(Some(res))
}
