use rbatis::rbdc::datetime::DateTime;
use crate::entity::{{table_name}}_entity::{{class_name}}Entity;
use crate::entity::sys_user_entity::SysUser;
use crate::GLOBAL_DB;
use crate::mapper::{{business_name}}_mapper;
use crate::model::common_model::Page;
use crate::model::{{business_name}}_model::{{class_name}}List;
use crate::utils::func::{create_page, create_page_list, is_modify_ok};

pub async fn get_{{business_name}}_page(page_num:u64,page_size:u64,{{business_name}}_code:Option<String>,{{business_name}}_name:Option<String>,status:Option<String>)->rbatis::Result<Page<{{class_name}}List>>{
    let (num,size) = create_page(page_num,page_size);
    let list = {{business_name}}_mapper::get_{{business_name}}_page(&mut GLOBAL_DB.clone(),num,size,{{business_name}}_code.clone(),{{business_name}}_name.clone(),status.clone()).await?;
    let total = {{business_name}}_mapper::get_{{business_name}}_count(&mut GLOBAL_DB.clone(),{{business_name}}_code,{{business_name}}_name,status).await?;
    Ok(create_page_list(list,total))
}

pub async fn {{business_name}}_add_post(user_id:i32,{{business_name}}_code:Option<String>,{{business_name}}_name:Option<String>,{{business_name}}_sort:Option<i8>,status:Option<String>,remark:Option<String>)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let {{table_name}} = {{class_name}}Entity{
        {{business_name}}_id: None,
        {{business_name}}_code,
        {{business_name}}_name,
        {{business_name}}_sort,
        status,
        create_by: Some(user.user_name.clone()),
        create_time: Some(DateTime::now()),
        update_by: None,
        update_time: None,
        remark
    };
    let rows = {{class_name}}Entity::insert(&mut GLOBAL_DB.clone(),&{{table_name}}).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn get_{{business_name}}_by_id(id:i64)->rbatis::Result<Option<{{class_name}}List>>{
    let list = {{business_name}}_mapper::get_{{business_name}}_by_id(&mut GLOBAL_DB.clone(),id).await?;
    let one = list.get(0).cloned();
    Ok(one)
}


pub async fn {{business_name}}_edit_post(user_id:i32,{{business_name}}_id:Option<i64>,{{business_name}}_code:Option<String>,{{business_name}}_name:Option<String>,{{business_name}}_sort:Option<i8>,status:Option<String>,remark:Option<String>)->rbatis::Result<bool>{
    let user = SysUser::select_by_column(&mut GLOBAL_DB.clone(), "user_id", user_id).await?;
    let user = user.get(0).unwrap();
    let {{table_name}} = {{class_name}}Entity{
        {{business_name}}_id,
        {{business_name}}_code,
        {{business_name}}_name,
        {{business_name}}_sort,
        status,
        create_by: None,
        create_time: None,
        update_by: Some(user.user_name.clone()),
        update_time: Some(DateTime::now()),
        remark
    };
    let rows = {{class_name}}Entity::update_by_column(&mut GLOBAL_DB.clone(),&{{table_name}},"{{business_name}}_id").await?;
    Ok(is_modify_ok(rows.rows_affected))
}

pub async fn del_{{business_name}}_by_id({{business_name}}_id:String)->rbatis::Result<bool>{
    let rows = {{business_name}}_mapper::del_{{business_name}}_by_id(&mut GLOBAL_DB.clone(),{{business_name}}_id).await?;
    Ok(is_modify_ok(rows.rows_affected))
}

