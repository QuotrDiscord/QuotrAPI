use actix_web::{cookie::Cookie, get, web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::ops::Add;

pub struct JwtSecret {
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
    permissions: String, // Permissions granted to token
}

#[get("/login/{code}")]
pub async fn login(path: web::Path<String>, jwt_secret: web::Data<JwtSecret>) -> impl Responder {
    let discord_code: String = path.into_inner();
    let secret: &[u8] = jwt_secret.secret.as_bytes();
    let iat_timestamp: usize = chrono::offset::Utc::now().timestamp() as usize;
    let exp_timestamp: usize = iat_timestamp.clone().add(2_592_000);

    let my_claims = Claims {
        exp: exp_timestamp,
        iat: iat_timestamp,
        sub: discord_code.clone(),
        permissions: "Not yet supported".to_owned(),
    };

    // TODO: Add custom error handling to be able to use "?" syntax
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap();

    let cookie = Cookie::build("auth_token", token)
        .domain("localhost")
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).await
}
