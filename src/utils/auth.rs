// I basically got these auths from here already https://github.com/Bechma/realworld-leptos/blob/main/src/auth/server.rs
use axum::http::header;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
}

static AUTH_COOKIE: &str = "token";
pub(crate) static REMOVE_COOKIE: &str = "token=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT";

pub(crate) fn encode_token(token_claims: TokenClaims) -> jsonwebtoken::errors::Result<String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token_claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub(crate) fn decode_token(
    token: &str
) -> Result<jsonwebtoken::TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

#[tracing::instrument]
pub(crate) fn get_token() -> Option<String> {
    if let Some(req) = leptos::context::use_context::<axum::http::request::Parts>() {
        req.headers
            .get(header::COOKIE)
            .and_then(|cookie_header| {
                cookie_header
                    .to_str()
                    .ok()
                    .and_then(|cookies| {
                        cookies
                            .split(';')
                            .find(|cookie| cookie.trim().starts_with("token="))
                            .and_then(|cookie| cookie.split('=').nth(1))
                    })
                    .and_then(|token| decode_token(token).ok())
                    .map(|jwt| jwt.claims.sub)
    		})
    } else {
        None
    }
}

#[tracing::instrument]
pub async fn set_token(user_id: String) -> bool {
    if let Some(res) = leptos::context::use_context::<leptos_axum::ResponseOptions>() {
        let token = encode_token(TokenClaims {
            sub: user_id,
            exp: (sqlx::types::chrono::Utc::now().timestamp() as usize) + 3_600_000,
        })
            .unwrap();
        res.insert_header(
            header::SET_COOKIE,
            header::HeaderValue::from_str(&format!("{AUTH_COOKIE}={token}; path=/; HttpOnly"))
                .expect("header value couldn't be set"),
        );
        true
    } else {
        false
    }
}
