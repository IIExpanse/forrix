use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Default)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    // informational
    #[default]
    #[command(description = "Display this text.")]
    Start,
    #[command(description = "Show system health status.")]
    GetSystemHealth,
    #[command(description = "Show all properties and their values.")]
    GetCurrentConfig,
    #[command(description = "Show detailed information about chosen instrument.")]
    GetChosenInstrumentInfo { ticker: String },
    // configurational
    #[command(description = "Set instrument to be traded.")]
    SetInstrument { ticker: String },
    #[command(description = "Set market identifier code for chosen stock exchange.")]
    SetMic { mic: String },
    #[command(description = "Set amount to be traded in 1 operation.")]
    SetAmount { amount: u32 },
    // trade operations
    #[command(description = "Execute buy operation.")]
    Buy,
    #[command(description = "Execute sell operation.")]
    Sell,
    #[command(description = "Close current position.")]
    ClosePosition,
}
