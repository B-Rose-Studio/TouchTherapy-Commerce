use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenJWT(pub String);

impl TokenJWT {
    pub fn new<T: ToString>(key: &str, duration: i64, content: &T) -> Self {
        let content = content.to_string();
        let now = Utc::now();
        let exp = now + Duration::hours(duration);

        let key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes()).unwrap();
        let header = Header {
            algorithm: jwt::AlgorithmType::Hs384,
            ..Default::default()
        };
        let mut claims = BTreeMap::new();

        claims.insert("sub", content);
        claims.insert("iat", now.to_rfc3339());
        claims.insert("exp", exp.to_rfc3339());

        let token = Token::new(header, claims).sign_with_key(&key).unwrap();
        Self(token.as_str().to_owned())
    }

    pub fn validate<T: FromStr>(&self, key: &str) -> Option<T> {
        let key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes()).ok()?;
        let token: Token<Header, BTreeMap<String, String>, _> =
            self.0.verify_with_key(&key).ok()?;

        let _header = token.header();
        let claims = token.claims();

        let exp_str = claims["exp"].clone();
        let content_str = claims["sub"].clone();

        let now = Utc::now();
        let exp: DateTime<Utc> = DateTime::from_str(&exp_str).ok()?;

        if now >= exp {
            None
        } else {
            T::from_str(&content_str).ok()
        }
    }
}
