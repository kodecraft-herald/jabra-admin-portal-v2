use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginRequest {
    pub email: String,
    pub password: String,
}

impl DirectusLoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginResponse {
    pub data: DirectusLoginResponseData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DirectusLoginResponseData {
    pub access_token: String,
    pub expires: i64,
    pub refresh_token: String,
}
