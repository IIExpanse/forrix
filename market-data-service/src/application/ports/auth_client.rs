use shared::messages::grpc::tradeapi::v1::auth::{self, auth_service_client};
use std::sync::{Arc, RwLock};

use tonic::transport::Channel;
use tracing::{debug, error, info, warn};

use crate::application::errors::app_errors::AppError;

pub struct FinamAuthGrpcClient;

impl FinamAuthGrpcClient {
    pub async fn create_grpc_client(
        api_url: &str,
    ) -> Result<auth_service_client::AuthServiceClient<Channel>, AppError> {
        info!("Initializing gRPC auth connection to {}", api_url);
        let client = auth_service_client::AuthServiceClient::connect(api_url.to_owned()).await;

        match client {
            Ok(res) => Ok(res),
            Err(err) => Err(AppError {
                message: "Failed to create auth client.".to_owned(),
                cause: Some(Box::new(err)),
            }),
        }
    }

    pub async fn start_auth_flow(
        mut client: auth_service_client::AuthServiceClient<Channel>,
        api_token: &str,
        token_holder: Arc<RwLock<String>>,
    ) -> Result<(), AppError> {
        info!("Getting initial jwt_token.");

        let res = client
            .auth(auth::AuthRequest {
                secret: api_token.to_string(),
                source_app_id: "market-data-service".to_owned(),
            })
            .await;

        let jwt_token = match res {
            Ok(resp) => resp.into_inner().token,
            Err(err) => {
                return Err(AppError {
                    message: "Failed to authenticate".to_owned(),
                    cause: Some(Box::new(err)),
                });
            }
        };
        *token_holder.write().unwrap() = jwt_token;

        tokio::spawn(Self::start_jwt_renewal(
            client,
            api_token.to_string(),
            token_holder,
        ));

        Ok(())
    }

    async fn start_jwt_renewal(
        mut client: auth_service_client::AuthServiceClient<Channel>,
        api_token: String,
        token_holder: Arc<RwLock<String>>,
    ) -> Result<(), AppError> {
        info!("Starting token renewal process.");

        let request = auth::SubscribeJwtRenewalRequest {
            secret: api_token.clone(),
            source_app_id: "market_data_service".to_owned(),
        };
        let mut attempts = 0;

        loop {
            debug!("Attempting to start jwt renewal stream..");

            match client.subscribe_jwt_renewal(request.clone()).await {
                Ok(resp) => {
                    debug!("Stream started.");

                    let mut stream = resp.into_inner();
                    attempts = 0;

                    loop {
                        match stream.message().await {
                            Ok(res) => match res {
                                Some(jwt_token_response) => {
                                    debug!("Received new jwt token.");

                                    *token_holder.write().map_err(|e| AppError {
                                        message: format!(
                                            "Failure while acquiring write lock: {}",
                                            e
                                        ),
                                        cause: None,
                                    })? = jwt_token_response.token;
                                }
                                None => {
                                    warn!("Received empty token response.");
                                }
                            },
                            Err(err) => {
                                error!("Error while awaiting next jwt renewal message: {}", err);
                                break;
                            }
                        }
                    }
                }
                Err(error) => {
                    error!(
                        "Error while attempting to subscribe to jwt renewal: {}",
                        error
                    );
                    attempts += 1;

                    if attempts == 5 {
                        error!("Connection attempts exceeded, stopping token renewal.");
                        return Err(AppError {
                            message: "Connection attempts exceeded for token renewal.".to_owned(),
                            cause: None,
                        });
                    }
                }
            }
        }
    }
}
