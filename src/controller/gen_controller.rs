use salvo::{oapi::endpoint, Depot, Writer};
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::Request;
use crate::model::common_model::Page;
use crate::model::gen_table_model::{
    GenTableList, GenTableListPayload, DbTableList, TableInfo, GenTableModifyPayload
};
use crate::service::gen_table_service;
use crate::utils::res::{match_no_res_ok, match_ok, Res, ResObj};

/// 列出gen_table表行项目
#[endpoint(
    tags("代码生成"),
    parameters(
        GenTableListPayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<GenTableList>>,description ="数据表列表")
    ),
)]
pub async fn get_gen_table_page(req:&mut Request)->Res<Page<GenTableList>>{
    let payload:GenTableListPayload = req.parse_queries().unwrap();
    match_ok(
        gen_table_service::get_gen_table_page(
            payload.page_num,payload.page_size,
            payload.table_name,payload.table_comment,
            payload.create_time
        ).await
    )
}

/// 根据表id获取表gen_table行项目
#[endpoint(
    tags("代码生成"),
    responses(
        (status_code = 200,body=ResObj<Option<TableInfo>>,description ="数据表详情")
    ),
)]
pub async fn get_gen_table_by_id(id: PathParam<String>) ->Res<Option<TableInfo>> {
    match_ok(gen_table_service::get_gen_table_by_id(id.into_inner()).await)
}

/// 修改岗位
#[endpoint(
    tags("代码生成"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="修改数据表")
    ),
)]
pub async fn put_edit_gen_table(payload:JsonBody<GenTableModifyPayload>,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let table = payload.into_inner();
    match_no_res_ok(gen_table_service::put_edit_gen_table(user_id, table).await)
}

/// 根据表id删除表gen_table行项目
#[endpoint(
    tags("代码生成"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="删除数据表")
    ),
)]
pub async fn del_gen_table_by_id(id: PathParam<String>)->Res<()>{
    match_no_res_ok(gen_table_service::del_gen_table_by_id(id.into_inner()).await)
}

/// 列出数据库里的表
#[endpoint(
    tags("代码生成"),
    parameters(
        GenTableListPayload
    ),
    responses(
        (status_code = 200,body=ResObj<Page<GenTableList>>,description ="数据库列表")
    ),
)]
pub async fn get_db_table_page(req:&mut Request)->Res<Page<DbTableList>>{
    let payload:GenTableListPayload = req.parse_queries().unwrap();
    match_ok(gen_table_service::get_db_table_page(payload.page_num,payload.page_size).await)
}

/// 添加数据表
#[endpoint(
    tags("代码生成"),
    responses(
        (status_code = 200,body=ResObj<()>,description ="添加数据表")
    ),
)]
pub async fn post_import_tables(req:&mut Request,depot:&mut Depot)->Res<()>{
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let param = req.query::<String>("tables").unwrap();
    let table_names = format!("('{}')", param.replace(",", "','"));
    // Query tables info in database
    let mut table_list = gen_table_service::get_db_table_by_names(table_names)
        .await.unwrap();
    match_no_res_ok(gen_table_service::import_tables(user_id, &mut table_list).await)
}
