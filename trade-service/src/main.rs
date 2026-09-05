use std::env;

use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::application::security::auth_manager::AuthManager;
use crate::application::subscription::finam::subscriber::FinamBarsSubscriber;
use crate::application::subscription::finam::subscriber::Subscriber;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    let api_url = env::var("API_URL").expect("API_URL must be provided.");
    let api_token = env::var("API_TOKEN").expect("API_TOKEN must be provided.");

    let auth_manager = AuthManager::new(&api_url, &api_token).await;
    let mut bars_subscriber = FinamBarsSubscriber::new(&api_url, &auth_manager).await?;
    bars_subscriber.subscribe().await?;

    tokio::signal::ctrl_c().await.unwrap();
    println!("Received shutdown signal, exiting..");

    return Ok(());
}
