use actix_web::{get, web::ServiceConfig};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

pub fn main() -> impl for<'a> Fn(&'a mut ServiceConfig) + Send + Clone {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    config
}
