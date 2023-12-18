use actix_cors::Cors;
use actix_web::web::{self, ServiceConfig};
use discord_helpers::{DiscordApiCaller, JwtSecret};

mod discord_helpers;
mod routes;

pub fn main(
    jwt_secret: String,
    discord_endpoint: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
) -> impl for<'a> FnOnce(&'a mut ServiceConfig) + Send + Clone {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(JwtSecret { secret: jwt_secret }))
            .app_data(web::Data::new(DiscordApiCaller {
                api_endpoint: discord_endpoint,
                client_id: client_id,
                client_secret: client_secret,
                redirect_uri: redirect_uri,
            }))
            .service(routes::other::hello_world)
            .service(
                web::scope("/api")
                    .service(routes::auth::login)
                    .wrap(Cors::permissive()),
            );
    };

    config
}
