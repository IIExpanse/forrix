use shared::messages::grpc::tradeapi::v1::auth::{self, auth_service_client};

use tonic::transport::Channel;

pub struct FinamAuthGrpcClient {
    client: auth_service_client::AuthServiceClient<Channel>
}

impl FinamAuthGrpcClient {
    pub async fn create(api_url: String) -> Result<FinamAuthGrpcClient, tonic::transport::Error> {
        let instance = FinamAuthGrpcClient {
            client: auth_service_client::AuthServiceClient::connect(api_url).await?
        };
        Ok(instance)
    }
}

pub trait AuthClient {
    // todo introduce a generic error
    async fn authenticate(self, secret: &str) -> Result<String, tonic::Status>;
}

impl AuthClient for FinamAuthGrpcClient {
    async fn authenticate(mut self, secret: &str) -> Result<String, tonic::Status> {
        let jwt_token = self.client
            .auth(auth::AuthRequest {
                secret: secret.to_string(),
                source_app_id: "market-data-service".to_owned(),
            })
            .await?
            .into_inner()
            .token;

        Ok(jwt_token)
    }
}