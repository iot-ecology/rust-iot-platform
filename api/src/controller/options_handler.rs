use rocket::http::Method;
use rocket::options;

#[options("/<_..>")]
pub fn all_options() {
    // 空函数体，响应会被 CORS fairing 处理
}
