use shared::messages::grpc::tradeapi::v1::marketdata::SubscribeBarsRequest;
use shared::messages::grpc::tradeapi::v1::marketdata::TimeFrame;
use shared::messages::grpc::tradeapi::v1::marketdata::market_data_service_client;
use tonic::Request;
use tonic::metadata::MetadataValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        market_data_service_client::MarketDataServiceClient::connect("http://api.finam.ru:443")
            .await?;

    let payload = SubscribeBarsRequest {
        symbol: "IMOEXF".to_string(),
        timeframe: TimeFrame::M1 as i32,
    };

    let mut request = Request::new(payload);

    let token: MetadataValue<_> = "token_here".parse().unwrap();
    request.metadata_mut().insert("secret", token);

    let mut response_stream = client.subscribe_bars(request).await?.into_inner();

    loop {
        let res = response_stream.message().await;

        match res {
            Ok(resp) => match resp {
                Some(body) => {
                    println!("RECEIVED BARS RESPONSE: {:#?}", body.bars);
                }
                None => {}
            },
            Err(status) => {
                eprintln!("gRPC error encountered: {}", status);
                break;
            }
        }
    }

    return Ok(());
}
