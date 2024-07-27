use prost::Message;

#[derive(Message)]
pub struct SignUpResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

#[derive(Message)]
pub struct LoginResponse {
    #[prost(string, tag = "1")]
    pub token: String,
}

#[derive(Message)]
pub struct CreateTokenResponse {
    #[prost(string, tag = "1")]
    pub token: String,
}

#[derive(Message)]
pub struct VerifyTokenResponse {
    #[prost(string, tag = "1")]
    pub user_id: String,
}