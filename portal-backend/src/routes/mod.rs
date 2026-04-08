pub mod admin;
pub mod proxy;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(admin::configure)
            .configure(proxy::configure),
    );
}
