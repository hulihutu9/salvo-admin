use salvo::{oapi::endpoint,Writer};
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::{Depot, Request};
use crate::model::common_model::Page;
use crate::model::{{business_name}}_model::{ {{class_name}}List, {{class_name}}ListPayload, {{class_name}}ModifyPayload};
use crate::service::{{business_name}}_service;
use crate::utils::res::{match_no_res_ok, match_ok, Res, ResObj};

/// {{function_name}}列表
#[endpoint(
    tags("{{function_name}}"),
    parameters(
        {{class_name}}ListPayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<{{class_name}}List>>,description ="{{function_name}}列表")
    ),
)]
pub async fn get_{{business_name}}_page(req:&mut Request)->Res<Page<{{class_name}}List>>{
    let payload:{{class_name}}ListPayload = req.parse_queries().unwrap();
    match_ok({{business_name}}_service::get_{{business_name}}_page(payload.page_num,payload.page_size,payload.{{business_name}}_code,payload.{{business_name}}_name,payload.status).await)
}

/// 添加{{function_name}}
#[endpoint(
    tags("{{function_name}}"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加{{function_name}}")
    ),
)]
pub async fn {{business_name}}_add_post(payload:JsonBody<{{class_name}}ModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok({{business_name}}_service::{{business_name}}_add_post(user_id, payload.{{business_name}}_code.clone(), payload.{{business_name}}_name.clone(), payload.{{business_name}}_sort, payload.status.clone(), payload.remark.clone()).await)
}

/// 修改{{function_name}}
#[endpoint(
    tags("{{function_name}}"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改{{function_name}}")
    ),
)]
pub async fn put_edit_post(payload:JsonBody<{{class_name}}ModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    match_no_res_ok({{business_name}}_service::{{business_name}}_edit_post(user_id,payload.{{business_name}}_id, payload.{{business_name}}_code.clone(), payload.{{business_name}}_name.clone(), payload.{{business_name}}_sort, payload.status.clone(), payload.remark.clone()).await)
}

/// {{function_name}}详情
#[endpoint(
    tags("{{function_name}}"),
    responses(
        (status_code = 200,body=ResObj<Option<{{class_name}}List>>,description ="{{function_name}}详情")
    ),
)]
pub async fn get_{{business_name}}_by_id(id:PathParam<i64>)->Res<Option<{{class_name}}List>>{
    match_ok({{business_name}}_service::get_{{business_name}}_by_id(id.into_inner()).await)
}

/// 删除{{function_name}}
#[endpoint(
    tags("{{function_name}}"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除{{function_name}}")
    ),
)]
pub async fn del_{{business_name}}_by_id(id:PathParam<String>)->Res<()>{
    match_no_res_ok({{business_name}}_service::del_{{business_name}}_by_id(id.into_inner()).await)
}
