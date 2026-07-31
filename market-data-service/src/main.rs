use hyper::Uri;
use shared::messages::grpc::tradeapi::v1::auth::AuthRequest;
use shared::messages::grpc::tradeapi::v1::auth::SubscribeJwtRenewalRequest;
use shared::messages::grpc::tradeapi::v1::auth::auth_service_client;
use shared::messages::grpc::tradeapi::v1::marketdata::SubscribeBarsRequest;
use shared::messages::grpc::tradeapi::v1::marketdata::TimeFrame;
use shared::messages::grpc::tradeapi::v1::marketdata::market_data_service_client;
use std::env;
use std::str::FromStr;
use std::sync::RwLock;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

mod application;
mod infrastructure;
mod domain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = env::var("API_URL").expect("API_URL must be provided.");
    let api_token = env::var("API_TOKEN").expect("API_TOKEN must be provided.");

    let mut auth_client = auth_service_client::AuthServiceClient::connect(api_url.clone()).await?;
    let jwt_token = auth_client
        .auth(AuthRequest {
            secret: api_token.clone(),
            source_app_id: "market-data-service".to_owned(),
        })
        .await?
        .into_inner()
        .token;

    let jwt_token_lock = RwLock::new(jwt_token);

    let channel = Channel::builder(Uri::from_str(&api_url).expect("API_URL must be valid."))
        .connect()
        .await?;

    let auth_func = |mut req: Request<()>| {
        let token_guard = jwt_token_lock.read().unwrap();

        let bearer_token: MetadataValue<_> =
            MetadataValue::try_from("Bearer ".to_owned() + &token_guard)
                .expect("Only visible ASCII characters (32-127) are permitted.");

        req.metadata_mut().insert("Authorization", bearer_token);

        Ok(req)
    };

    let mut market_data_client =
        market_data_service_client::MarketDataServiceClient::with_interceptor(channel, auth_func);

    let sub_request = SubscribeBarsRequest {
        symbol: "IMOEXF".to_string(),
        timeframe: TimeFrame::M1 as i32,
    };

    let request = Request::new(sub_request);

    let mut response_stream = market_data_client
        .subscribe_bars(request)
        .await?
        .into_inner();

    let handle = tokio::spawn(start_jwt_renewal(auth_client, api_token, jwt_token_lock));

    loop {
        let res = response_stream.message().await;

        match res {
            Ok(resp) => match resp {
                Some(body) => {
                    println!("Received bars response: {:#?}", body.bars);
                }
                None => {}
            },
            Err(status) => {
                eprintln!("Error while subscribing to bars: {}", status);
                break;
            }
        }
    }
    handle.abort();

    return Ok(());
}

async fn start_jwt_renewal(
    mut client: auth_service_client::AuthServiceClient<Channel>,
    api_token: String,
    jwt_token_lock: RwLock<String>,
) {
    let request = SubscribeJwtRenewalRequest {
        secret: api_token.clone(),
        source_app_id: "market_data_service".to_owned(),
    };
    let mut attempts = 0;

    loop {
        println!("Attempting to start jwt renewal stream");

        match client.subscribe_jwt_renewal(request.clone()).await {
            Ok(resp) => {
                let mut stream = resp.into_inner();
                attempts = 0;

                loop {
                    match stream.message().await {
                        Ok(res) => match res {
                            Some(jwt_token_response) => {
                                *jwt_token_lock.write().unwrap() = jwt_token_response.token;
                            }
                            None => {}
                        },
                        Err(err) => {
                            eprintln!("Error while awaiting next jwt renewal message: {}", err);
                            break;
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!(
                    "Error while attempting to subscribe to jwt renewal: {}",
                    error
                );
                attempts += 1;

                if attempts == 5 {
                    eprintln!("Connection attempts exceeded, stopping token renewal.");
                    break;
                }
            }
        }
    }
}
