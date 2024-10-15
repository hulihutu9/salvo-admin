use rbatis::rbdc::datetime::DateTime;
use crate::entity::gen_table_entity::GenTableEntity;
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::gen_table_mapper;
use crate::model::common_model::Page;
use crate::model::gen_table_model::{DbTableList, GenTableList};
use crate::utils::func::{create_page, create_page_list, is_modify_ok};

pub async fn get_gen_table_page(
    page_num:u64,page_size:u64,table_name:Option<String>,
    table_comment:Option<String>,begin_time:Option<DateTime>
) ->rbatis::Result<Page<GenTableList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = gen_table_mapper::get_gen_table_page(&mut GLOBAL_DB.clone(),num,size,
                                    table_name.clone(),table_comment.clone(),begin_time.clone())
                                    .await?;
    let total = gen_table_mapper::get_gen_table_count(&mut GLOBAL_DB.clone(),table_name,
                       table_comment,begin_time.clone()).await?;
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

pub async fn get_db_table_page(page_num:u64,page_size:u64) ->rbatis::Result<Page<DbTableList>>{
    let (num,size) = create_page(page_num,page_size);
    let list = gen_table_mapper::get_db_table_page(
        &mut GLOBAL_DB.clone(),num,size).await?;
    let total = list.len() as u64;
    Ok(create_page_list(list,total))
}
