use salvo::Router;
use crate::controller::user_controller;

pub fn init_router()->Router{
    let router = Router::new();
    router.push(
        Router::with_path("/getInfo").get(user_controller::get_info)
    ).push(
        Router::with_path("/getRouters").get(user_controller::get_routers)
    )
}

pub fn init_router_no_token()->Router{
    let router = Router::new();
    router.push(
                // 验证码
        Router::with_path("/captchaImage").get(user_controller::get_captcha)
    ).push(
                // 登录
        Router::with_path("/login").post(user_controller::login)
    ).push(
                // 退出登录
        Router::with_path("/logout").post(user_controller::log_out)
    )
}