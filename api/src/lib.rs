use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, ServiceConfig},
};
use auth_routes::JwtSecret;

mod auth_routes;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

pub fn main(jwt_secret: String) -> impl for<'a> FnOnce(&'a mut ServiceConfig) + Send + Clone {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(JwtSecret { secret: jwt_secret }))
            .service(
                web::scope("/api")
                    .service(hello_world)
                    .service(auth_routes::login)
                    .wrap(Cors::permissive()),
            );
    };

    config
}
