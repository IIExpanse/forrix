use hyper::Uri;
use shared::{
    application::errors::app_errors::AppError,
    proto::finam::grpc::tradeapi::v1::marketdata::{
        Bar, SubscribeBarsRequest, TimeFrame,
        market_data_service_client::{self, MarketDataServiceClient},
    },
};
use std::{str::FromStr, time::Duration};
use tokio::{task::JoinHandle, time::sleep};
use tonic::{Request, service::interceptor::InterceptedService, transport::Channel};
use tracing::{error, warn};

use crate::application::security::auth_manager::{AuthInterceptor, AuthManager};

// todo: move to parent module
pub trait Subscriber<T> {
    async fn subscribe(&mut self) -> Result<(), AppError>;

    fn on_message(&self, message: T);
}

pub struct FinamBarsSubscriber {
    market_data_client: MarketDataServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    subscription_handle: Option<JoinHandle<Result<(), AppError>>>,
}

impl FinamBarsSubscriber {
    pub async fn new(
        api_url: &str,
        auth_manager: &AuthManager,
    ) -> Result<FinamBarsSubscriber, AppError> {
        let channel = Channel::builder(Uri::from_str(api_url).expect("API_URL must be valid."))
            .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
            .map_err(|e| AppError {
                message: "Failed to configure tls for MarketDataSubscriber".to_owned(),
                cause: Some(Box::new(e)),
            })?
            .connect()
            .await
            .map_err(|e| AppError {
                message: "Failed to open gRPC channel for MarketDataSubscriber".to_owned(),
                cause: Some(Box::new(e)),
            })?;

        let market_data_client =
            market_data_service_client::MarketDataServiceClient::with_interceptor(
                channel,
                auth_manager.get_auth_interceptor(),
            );
        Ok(FinamBarsSubscriber {
            market_data_client,
            subscription_handle: None,
        })
    }
}

impl Subscriber<&Bar> for FinamBarsSubscriber {
    async fn subscribe(&mut self) -> Result<(), AppError> {
        let sub_request = SubscribeBarsRequest {
            symbol: "IMOEXF@RTSX".to_string(),
            timeframe: TimeFrame::M1 as i32,
        };

        let mut attempts = 0;

        loop {
            let request = Request::new(sub_request.clone());

            let resp = self
                .market_data_client
                .subscribe_bars(request)
                .await
                .map_err(|e| AppError {
                    message: "Failed to subscribe to bars".to_owned(),
                    cause: Some(Box::new(e)),
                });

            if let Err(e) = resp {
                error!("Error while subscribing to bars: {}", e);
                attempts += 1;
                sleep(Duration::from_millis(200)).await;

                if attempts >= 5 {
                    error!("Connection attempts exceeded for bars stream, aborting");
                    return Err(e);
                }
                continue;
            }
            attempts = 0;

            let mut bars_stream = resp.unwrap().into_inner();

            loop {
                let res = bars_stream.message().await;

                match res {
                    Ok(resp) => match resp {
                        Some(body) => {
                            body.bars.iter().for_each(|bar| self.on_message(bar));
                        }
                        None => {
                            warn!("Received empty bars response.")
                        }
                    },
                    Err(status) => {
                        error!("Error while waiting for bars message: {}", status);
                        break;
                    }
                }
            }
        }
    }

    fn on_message(&self, message: &Bar) {
        println!("Received bar: {:#?}", message);
    }
}

impl Drop for FinamBarsSubscriber {
    fn drop(&mut self) {
        if let Some(handle) = &self.subscription_handle {
            handle.abort()
        }
    }
}
