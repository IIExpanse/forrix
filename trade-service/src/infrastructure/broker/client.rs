use shared::application::errors::app_errors::AppError;

pub enum Side {
    Buy,
    Sell,
}

pub trait TradeClientFacade {
    async fn place_immediate_order(
        ticker: &str,
        mic: &str,
        side: Side,
        price: u64, // in minimal tick units (e.g., microcents)
        amount_lots: u64,
    ) -> Result<(), AppError>;
}
