use shared::application::errors::app_errors::AppError;
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;
use tonic::Status;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;

use tracing::info;

use crate::infrastructure::broker::finam::auth_client;

pub struct AuthManager {
    token_holder: Arc<RwLock<String>>,
    handle: JoinHandle<Result<(), AppError>>,
}

pub struct AuthInterceptor {
    token_holder: Arc<RwLock<String>>,
}

impl AuthManager {
    pub async fn new(api_url: &str, api_token: &str) -> Self {
        let token_holder = Arc::new(RwLock::new("".to_owned()));

        let client = match auth_client::FinamAuthGrpcClient::create_client(api_url).await {
            Ok(client) => client,
            Err(e) => panic!("Failed to create auth client: {}", e),
        };

        let handle = match auth_client::FinamAuthGrpcClient::start_auth_flow(
            client,
            api_token,
            token_holder.clone(),
        )
        .await
        {
            Ok(res) => {
                info!("Successfully started authentication process.");
                res
            }
            Err(e) => panic!("Failed to initialize authentication flow: {}", e),
        };

        AuthManager {
            token_holder,
            handle,
        }
    }

    pub fn get_auth_interceptor(&self) -> AuthInterceptor {
        AuthInterceptor {
            token_holder: self.token_holder.clone(),
        }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let token = match self.token_holder.read() {
            Ok(res) => res.to_owned(),
            Err(e) => panic!("There is a PoisonError in token lock: {}", e),
        };

        let bearer_token: MetadataValue<_> = MetadataValue::try_from("Bearer ".to_owned() + &token)
            .expect("Only visible ASCII characters (32-127) are permitted.");

        request.metadata_mut().insert("authorization", bearer_token);

        Ok(request)
    }
}

impl Drop for AuthManager {
    fn drop(&mut self) {
        self.handle.abort()
    }
}
