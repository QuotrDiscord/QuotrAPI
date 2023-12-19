use actix_web::{cookie::Cookie, cookie::SameSite, get, web, HttpResponse, Responder};

use crate::discord_helpers;
use crate::discord_helpers::{AccessTokenResponse, DiscordApiCaller, JwtSecret};
use crate::web_helpers::WebDomain;

#[get("/login/{code}")]
pub async fn login(
    path: web::Path<String>,
    jwt_secret: web::Data<JwtSecret>,
    discord_caller_data: web::Data<DiscordApiCaller>,
    web_domain: web::Data<WebDomain>,
) -> impl Responder {
    let discord_code: String = path.into_inner();
    let jwt_bytes: &[u8] = jwt_secret.secret.as_bytes();
    let cloned_discord_caller_data: DiscordApiCaller = DiscordApiCaller {
        api_endpoint: discord_caller_data.api_endpoint.clone(),
        client_id: discord_caller_data.client_id.clone(),
        client_secret: discord_caller_data.client_secret.clone(),
        redirect_uri: discord_caller_data.redirect_uri.clone(),
    };

    let token_response: Option<AccessTokenResponse> =
        discord_helpers::fetch_discord_access_token(&cloned_discord_caller_data, discord_code)
            .await;

    if let Some(successful_response) = token_response {
        let token = discord_helpers::generate_jwt(successful_response.access_token, jwt_bytes);

        let cookie = Cookie::build("auth_token", token)
            .domain(&*web_domain.domain)
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .finish();

        HttpResponse::Ok().cookie(cookie).await
    } else {
        HttpResponse::InternalServerError().await
    }
}
