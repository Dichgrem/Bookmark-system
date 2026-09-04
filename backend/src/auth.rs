use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn sign_token(secret: &str, expire_hours: usize) -> Result<String, String> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + expire_hours * 3600;
    let claims = Claims { sub: 1, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Token 生成失败: {}", e))
}

fn verify_token(token: &str, secret: &str) -> Result<Claims, String> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| format!("Token 无效: {}", e))?;
    Ok(data.claims)
}

use crate::AppState;

pub struct AuthUser(pub i64);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header_value = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"code":401,"msg":"未登录","data":null})),
                )
            })?;

        let token = header_value.strip_prefix("Bearer ").ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"code":401,"msg":"认证格式错误","data":null})),
            )
        })?;

        let claims = verify_token(token, &state.jwt_secret).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"code":401,"msg":e,"data":null})),
            )
        })?;

        Ok(AuthUser(claims.sub))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_roundtrip() {
        let token = sign_token("secret123", 72).unwrap();
        let claims = verify_token(&token, "secret123").unwrap();
        assert_eq!(claims.sub, 1);
    }

    #[test]
    fn verify_wrong_secret() {
        let token = sign_token("correct", 72).unwrap();
        assert!(verify_token(&token, "wrong").is_err());
    }

    #[test]
    fn verify_tampered_token() {
        let token = sign_token("secret", 72).unwrap();
        let tampered = token + "x";
        assert!(verify_token(&tampered, "secret").is_err());
    }
}
