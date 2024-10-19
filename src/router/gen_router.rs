use salvo::Router;
use crate::controller::gen_controller;

pub fn init_router() ->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/tool/gen/list").get(gen_controller::get_gen_table_page)
    ).push(
        Router::with_path("/tool/gen/<id>").get(gen_controller::get_gen_table_by_id)
            .delete(gen_controller::del_gen_table_by_id)
    ).push(
        Router::with_path("/tool/gen/db/list").get(gen_controller::get_db_table_page)
    ).push(
        Router::with_path("/tool/gen/importTable").post(gen_controller::post_import_tables)
    ).push(
        Router::with_path("/tool/gen").put(gen_controller::put_edit_gen_table)
    )
}