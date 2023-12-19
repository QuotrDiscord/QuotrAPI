use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Add};

#[derive(Debug)]
pub struct JwtSecret {
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to, in this case, Discord user ID)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordApiCaller {
    pub api_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
}

pub async fn fetch_discord_access_token(
    caller_data: &DiscordApiCaller,
    oauth_code: String,
) -> Option<AccessTokenResponse> {
    let client: Client = Client::new();
    let mut form_data = HashMap::new();
    let redirect_uri = &*format!("{}/login", &*caller_data.domain);

    form_data.insert("grant_type", "authorization_code");
    form_data.insert("code", &*oauth_code);
    form_data.insert("redirect_uri", redirect_uri);

    let response = client
        .post(format!("{}", caller_data.api_endpoint))
        .basic_auth(
            caller_data.client_id.clone(),
            Some(caller_data.client_secret.clone()),
        )
        .form(&form_data)
        .send()
        .await
        .unwrap();

    let response_text = &*response.text().await.unwrap().clone();

    let atr: Option<AccessTokenResponse> = serde_json::from_str(response_text).ok();

    return atr;
}

pub fn generate_jwt(sub: String, secret: &[u8]) -> String {
    let iat_timestamp: usize = chrono::offset::Utc::now().timestamp() as usize;
    let exp_timestamp: usize = iat_timestamp.clone().add(2_592_000);
    let my_claims = Claims {
        exp: exp_timestamp,
        iat: iat_timestamp,
        sub: sub,
    };

    // TODO: Add custom error handling to be able to use "?" syntax
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap();

    return token;
}
