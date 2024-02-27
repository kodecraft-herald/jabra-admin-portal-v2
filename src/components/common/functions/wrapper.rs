use crate::errors::JabraError;
use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub async fn get_cookie_value(cookie_name: &str) -> Result<String, crate::errors::JabraError> {
    use axum::http::HeaderName;
    use std::str::FromStr;
    let cookie_value = use_context::<http::request::Parts>()
        .and_then(|req| {
            req.headers
                .get(HeaderName::from_str("cookie").unwrap())
                .and_then(|v| match v.to_str().ok() {
                    Some(r) => Some(r.to_string()),
                    None => None,
                })
        })
        .unwrap_or("".to_string());
    let cookies: Vec<&str> = cookie_value.split("; ").collect();

    let mut cookie_value = String::new();
    for cookie in cookies {
        let parts: Vec<&str> = cookie.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0] == cookie_name {
            cookie_value = parts[1].to_string();
        }
    }

    match cookie_value {
        value if value.len() > 0 => Ok(value),
        _ => Err(JabraError::CookieFetchError),
    }
}

#[server]
pub async fn check_server_cookie() -> Result<bool, ServerFnError> {
    let cookie_value = get_cookie_value("jabra-admin-portal-v2").await;
    match cookie_value {
        Ok(val) => {
            if val.len() > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        // Err(e) => {
        //     Err(ServerFnError::ServerError(e.to_string()))
        // }
        _ => Ok(false),
    }
}

pub enum HttpMethod {
    POST,
    GET,
    PATCH,
}
///Return Serialized response body
#[cfg(feature = "ssr")]
pub async fn call_and_parse<Request, Response>(
    request: Option<Request>,
    url: String,
    headers: reqwest::header::HeaderMap,
    method: HttpMethod,
) -> Result<Response, crate::errors::JabraError>
where
    Request: serde::Serialize,
    Response: Serializable,
{
    let client = reqwest::Client::new();
    let response = match method {
        HttpMethod::GET => {
            let path = match request {
                Some(req) => {
                    let query_string = serde_urlencoded::to_string(req);
                    match query_string {
                        Ok(query_string) => format!("{}?{}", url, query_string),
                        _ => url,
                    }
                }
                None => url,
            };
            client.get(&path).headers(headers).send().await
        }
        HttpMethod::POST => {
            client
                .post(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
        HttpMethod::PATCH => {
            client
                .patch(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
    };
    // log::info!("Response: {:?}", response);
    match response {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                let response = res.text().await.map_err(|e| JabraError::from(e))?;
                Response::de(&response).map_err(|e| JabraError::from(e))
            } else {
                Err(JabraError::APIResponseError(res.status().to_string()))
            }
        }
        Err(e) => Err(JabraError::from(e)),
    }
}

///Return boolean, true if httpstatus == 200, Error otherwise
#[cfg(feature = "ssr")]
pub async fn call<Request>(
    request: Option<Request>,
    url: String,
    headers: reqwest::header::HeaderMap,
    method: HttpMethod,
) -> Result<bool, crate::errors::JabraError>
where
    Request: serde::Serialize,
{
    let client = reqwest::Client::new();
    let response = match method {
        HttpMethod::GET => {
            let path = match request {
                Some(req) => {
                    let query_string = serde_urlencoded::to_string(req);
                    match query_string {
                        Ok(query_string) => format!("{}?{}", url, query_string),
                        _ => url,
                    }
                }
                None => url,
            };
            client.get(&path).headers(headers).send().await
        }
        HttpMethod::POST => {
            client
                .post(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
        HttpMethod::PATCH => {
            client
                .patch(url)
                .headers(headers)
                .json(&request.unwrap())
                .send()
                .await
        }
    };
    match response {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                Ok(true)
            } else {
                Err(JabraError::APIResponseError(res.status().to_string()))
            }
        }
        Err(e) => Err(JabraError::from(e)),
    }
}

#[cfg(feature = "ssr")]
pub fn enc(plain_text: String) -> String {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    let encryption_key = std::env::var("JABRAKEY").unwrap();
    let magic_crypt = new_magic_crypt!(encryption_key, 256);
    magic_crypt.encrypt_str_to_base64(plain_text)
}
#[cfg(feature = "ssr")]
pub fn dec(encrypted_text: String) -> String {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    let encryption_key = std::env::var("JABRAKEY").unwrap();
    let magic_crypt = new_magic_crypt!(encryption_key, 256);
    magic_crypt
        .decrypt_base64_to_string(encrypted_text)
        .unwrap()
}

#[cfg(feature = "ssr")]
pub async fn refresh_token(
    owner: String,
    refresh_token: String,
) -> Result<JabraCookie, crate::errors::JabraError> {
    use crate::components::models::DirectusLoginResponse;

    let url = std::env::var("DIRECTUSURL").unwrap();

    let path = format!("{}/auth/refresh", url);
    let json_body = serde_json::json!({
        "refresh_token": refresh_token,
        "mode": "json"
    });
    let client = reqwest::Client::new();
    let response = client
        .post(&path)
        .json(&json_body)
        .send()
        .await
        .map_err(|e| JabraError::from(e))?;

    if response.status().is_success() {
        let response_body = response.text().await.map_err(|e| JabraError::from(e))?;
        let directus_login_response =
            DirectusLoginResponse::de(&response_body).map_err(|e| JabraError::from(e));
        match directus_login_response {
            Ok(res) => {
                let expiration_time =
                    chrono::Utc::now().timestamp_millis() + res.data.expires - 60_000;
                let jabra_cookie = JabraCookie::new(
                    owner,
                    res.data.access_token,
                    res.data.refresh_token,
                    expiration_time,
                );
                Ok(jabra_cookie)
            }
            Err(e) => Err(JabraError::from(e)),
        }
    } else {
        Err(JabraError::APIResponseError(response.status().to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct JabraCookie {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

impl JabraCookie {
    pub fn new(
        user_id: String,
        access_token: String,
        refresh_token: String,
        expires_in: i64,
    ) -> Self {
        Self {
            user_id,
            access_token,
            refresh_token,
            expires_in,
        }
    }
    #[cfg(feature = "ssr")]
    pub fn encrypt(&self) -> String {
        let cookie_string = serde_json::to_string(self).unwrap();
        super::wrapper::enc(cookie_string)
    }
    #[cfg(feature = "ssr")]
    pub fn decrypt(encrypted_text: String) -> Result<Self, JabraError> {
        let decrypted_text = super::wrapper::dec(encrypted_text);
        match serde_json::from_str(&decrypted_text) {
            Ok(cookie) => Ok(cookie),
            Err(e) => {
                log::info!("CookieFetchError: {}", e);
                Err(JabraError::CookieFetchError)
            }
        }
    }
    #[cfg(feature = "ssr")]
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp_millis();
        now > self.expires_in
    }

    pub fn from_string(cookie_string: String) -> Result<Self, JabraError> {
        match serde_json::from_str(&cookie_string) {
            Ok(cookie) => Ok(cookie),
            Err(e) => {
                log::info!("CookieFetchError: {}", e);
                Err(JabraError::CookieFetchError)
            }
        }
    }
}
#[cfg(feature = "ssr")]
pub async fn set_jabra_cookie(jabra_cookie: Option<JabraCookie>, cookie_name: String) {
    use http::{header::SET_COOKIE, HeaderMap, HeaderValue};
    use leptos::*;
    use leptos_axum::{ResponseOptions, ResponseParts};
    let response =
        use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    let cookie_value = match jabra_cookie {
        Some(cookie) => cookie.encrypt(),
        None => "".to_string(),
    };
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "{}={}; HttpOnly; Path=/",
            cookie_name, cookie_value
        ))
        .expect("to create a header value"),
    );
    response_parts.headers = headers;
    response.overwrite(response_parts);
}
