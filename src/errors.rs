use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JabraError {
    #[serde(rename = "CookieFetchError")]
    CookieFetchError,
    #[serde(rename = "LoginError")]
    LoginError,
    #[serde(rename = "NoDataFoundError")]
    NoDataFoundError,
    #[serde(rename = "SerializationError")]
    SerializationError(String),
    #[serde(rename = "ReqwestError")]
    ReqwestError(String),
    #[serde(rename = "APIResponseError")]
    APIResponseError(String),
}
impl ToString for JabraError {
    fn to_string(&self) -> String {
        match self {
            JabraError::CookieFetchError => "Cookie not found".to_string(),
            JabraError::LoginError => "Username or Password does not matched".to_string(),
            JabraError::NoDataFoundError => "Data does not load correctly".to_string(),
            JabraError::SerializationError(e) => e.to_string(),
            JabraError::ReqwestError(e) => e.to_string(),
            JabraError::APIResponseError(message) => message.to_string(),
        }
    }
}

// Create From implementation for reqwest::Error, since it does not allow Clone
impl From<reqwest::Error> for JabraError {
    fn from(error: reqwest::Error) -> Self {
        JabraError::ReqwestError(error.to_string())
    }
}

impl From<SerializationError> for JabraError {
    fn from(error: SerializationError) -> Self {
        JabraError::SerializationError(error.to_string())
    }
}
