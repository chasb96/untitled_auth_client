use prost::Message;

#[derive(Message)]
pub struct SignUpRequest {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(string, tag = "2")]
    pub password: String,
}

#[derive(Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(string, tag = "2")]
    pub password: String,
}

#[derive(Message)]
pub struct CreateTokenRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
}

#[derive(Message)]
pub struct VerifyTokenRequest {
    #[prost(string, tag = "1")]
    pub token: String,
}