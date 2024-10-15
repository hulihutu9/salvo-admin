use salvo::Router;
use crate::controller::gen_controller;

pub fn init_router() ->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/tool/gen/list").get(gen_controller::get_gen_table_page)
    ).push(
        Router::with_path("/tool/gen/<id>").get(gen_controller::get_gen_table_by_id).delete(gen_controller::del_gen_table_by_id)
    )
}