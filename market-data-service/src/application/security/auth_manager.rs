use std::sync::{Arc, RwLock};

use tracing::info;

use crate::application::ports::auth_client;

pub struct AuthManager {
    token_holder: Arc<RwLock<String>>,
}

impl AuthManager {
    pub async fn new(api_url: &str, api_token: &str) -> Self {
        let instance = AuthManager {
            token_holder: Arc::new(RwLock::new("".to_owned())),
        };

        let client = match auth_client::FinamAuthGrpcClient::create_grpc_client(api_url).await {
            Ok(client) => client,
            Err(e) => panic!("Failed to create auth client: {}", e),
        };

        match auth_client::FinamAuthGrpcClient::start_auth_flow(
            client,
            api_token,
            instance.token_holder.clone(),
        )
        .await
        {
            Ok(_) => info!("Successfully started authentication process."),
            Err(e) => panic!("Failed to initialize authentication flow: {}", e),
        }

        instance
    }

    pub fn get_token(&self) -> String {
        match self.token_holder.read() {
            Ok(res) => res.to_owned(),
            Err(e) => panic!("There is a PoisonError in token lock: {}", e),
        }
    }

    pub fn get_token_holder(&self) -> Arc<RwLock<String>> {
        self.token_holder.clone()
    }
}
