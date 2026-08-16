use teloxide::{
    Bot,
    dispatching::{
        Dispatcher, HandlerExt, UpdateFilterExt, UpdateHandler,
        dialogue::{Dialogue, InMemStorage},
    },
    dptree,
    repls::CommandReplExt,
    requests::{Requester, ResponseResult},
    types::{Message, Update},
    utils::command::BotCommands,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

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

struct TgChat {
    ticker: Option<String>,
    mic: Option<String>,
    amount: Option<u32>,
}

// todo refactor one-arg setter functions into generic
impl TgChat {
    async fn start(bot: Bot, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;
        Ok(())
    }

    async fn set_instrument(&mut self, bot: Bot, msg: Message, ticker: String) -> HandlerResult {
        self.ticker = Some(ticker.clone());

        bot.send_message(msg.chat.id, format!("Saved: ticker = {}", ticker))
            .await?;

        Ok(())
    }

    async fn set_mic(&mut self, bot: Bot, msg: Message, mic: String) -> HandlerResult {
        self.mic = Some(mic.clone());

        bot.send_message(msg.chat.id, format!("Saved: mic = {}", mic))
            .await?;

        Ok(())
    }

    async fn set_amount(&mut self, bot: Bot, msg: Message, amount: u32) -> HandlerResult {
        self.amount = Some(amount);

        bot.send_message(msg.chat.id, format!("Saved: amount = {}", amount))
            .await?;

        Ok(())
    }

    // async fn commands_handler(bot: Bot, msg: Message, cmd: Command) {
    //     let handler = Update::filter_message().endpoint(|bot: Bot, msg: Message| async move {
    //         let cmd: Command = Command::parse(msg.text().unwrap(), "bot_username").unwrap();
    //         match cmd {
    //             Command::Start => todo!(),
    //             Command::GetSystemHealth => todo!(),
    //             Command::GetCurrentConfig => todo!(),
    //             Command::GetChosenInstrumentInfo { ticker } => todo!(),
    //             Command::SetInstrument { ticker } => todo!(),
    //             Command::SetMic { mic } => todo!(),
    //             Command::SetAmount { amount } => todo!(),
    //             Command::Buy => todo!(),
    //             Command::Sell => todo!(),
    //             Command::ClosePosition => todo!(),
    //         }
    //     });

    //     Dispatcher::builder(bot, handler)
    //         // Pass the shared state to the handler as a dependency.
    //         .dependencies(dptree::deps![messages_total])
    //         .enable_ctrlc_handler()
    //         .build()
    //         .dispatch()
    //         .await;
    // }
}
