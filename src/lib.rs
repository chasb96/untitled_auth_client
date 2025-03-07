mod error;
mod request;
mod response;

pub mod axum;

use std::{env, sync::OnceLock};

use prost::Message;
pub use request::*;
pub use response::*;
pub use error::Error;

use reqwest::{header::CONTENT_TYPE, Client};

static AUTH_CLIENT: OnceLock<AuthClient> = OnceLock::new();

pub struct AuthClient {
    http_client: Client,
    base_url: String,
}

impl AuthClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url
        }
    }

    pub async fn sign_up(&self, request: SignUpRequest) -> Result<SignUpResponse, Error> {
        let response = self.http_client
            .post(format!("{}/sign_up", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::from(response.status()));
        }

        let response_bytes = response.bytes().await?;

        let response = SignUpResponse::decode(response_bytes)?;

        Ok(response)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
        let response = self.http_client
            .post(format!("{}/login", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::from(response.status()));
        }

        let response_bytes = response.bytes().await?;

        let response = LoginResponse::decode(response_bytes)?;

        Ok(response)
    }

    pub async fn create_token(&self, request: CreateTokenRequest) -> Result<CreateTokenResponse, Error> {
        let response = self.http_client
            .post(format!("{}/create_token", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::from(response.status()));
        }

        let response_bytes = response.bytes().await?;

        let response = CreateTokenResponse::decode(response_bytes)?;

        Ok(response)
    }

    pub async fn verify_token(&self, request: VerifyTokenRequest) -> Result<VerifyTokenResponse, Error> {
        let response = self.http_client
            .post(format!("{}/verify_token", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::from(response.status()));
        }

        let response_bytes = response.bytes().await?;

        let response = VerifyTokenResponse::decode(response_bytes)?;

        Ok(response)
    }
}

impl Default for AuthClient {
    fn default() -> Self {
        let base_url = env::var("AUTH_BASE_URL")
            .unwrap_or("http://auth".to_string())
            .trim_end_matches('/')
            .to_string();
        
        Self { 
            http_client: Default::default(),
            base_url,
        }
    }
}