use std::sync::{Arc, Mutex};

use teloxide::{
    Bot,
    dispatching::{Dispatcher, UpdateFilterExt},
    dptree,
    requests::Requester,
    types::{Message, Update},
    utils::command::BotCommands,
};
use tracing::error;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone, Default, Debug)]
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

struct ChatState {
    ticker: Option<String>,
    mic: Option<String>,
    amount: Option<u32>,
}

struct TgChat;

// todo refactor one-arg setter functions into generic
impl TgChat {
    async fn start(bot: Bot, msg: Message, txt: String) -> HandlerResult {
        bot.send_message(msg.chat.id, txt).await?;
        Ok(())
    }

    async fn set_instrument(
        state_mutex: Arc<Mutex<ChatState>>,
        bot: Bot,
        msg: Message,
        ticker: String,
    ) -> HandlerResult {
        {
            state_mutex.lock().unwrap().ticker = Some(ticker.clone());
        }

        bot.send_message(msg.chat.id, format!("Saved: ticker = {}", ticker))
            .await?;

        Ok(())
    }

    async fn set_mic(
        state_mutex: Arc<Mutex<ChatState>>,
        bot: Bot,
        msg: Message,
        mic: String,
    ) -> HandlerResult {
        {
            state_mutex.lock().unwrap().mic = Some(mic.clone());
        }

        bot.send_message(msg.chat.id, format!("Saved: mic = {}", mic))
            .await?;

        Ok(())
    }

    async fn set_amount(
        state_mutex: Arc<Mutex<ChatState>>,
        bot: Bot,
        msg: Message,
        amount: u32,
    ) -> HandlerResult {
        {
            state_mutex.lock().unwrap().amount = Some(amount);
        }

        bot.send_message(msg.chat.id, format!("Saved: amount = {}", amount))
            .await?;

        Ok(())
    }

    async fn commands_handler(bot: Bot, state_mutex: Arc<Mutex<ChatState>>) {
        let handler = Update::filter_message().endpoint(
            |local_bot: Bot, local_msg: Message, state_mutex: Arc<Mutex<ChatState>>| async {
                let cmd: Command =
                    Command::parse(local_msg.text().unwrap(), "bot_username").unwrap();
                let res = match cmd {
                    Command::Start => {
                        Self::start(local_bot, local_msg, Command::descriptions().to_string()).await
                    }
                    Command::GetSystemHealth => todo!(),
                    Command::GetCurrentConfig => todo!(),
                    Command::GetChosenInstrumentInfo { ticker } => todo!(),
                    Command::SetInstrument { ticker } => todo!(),
                    Command::SetMic { mic } => todo!(),
                    Command::SetAmount { amount } => todo!(),
                    Command::Buy => todo!(),
                    Command::Sell => todo!(),
                    Command::ClosePosition => todo!(),
                };
                if let Err(err) = res {
                    error!("Error while execuring {:#?} command: {}", cmd, err);
                    return Err(err);
                };
                res
            },
        );

        Dispatcher::builder(bot, handler)
            // Pass the shared state to the handler as a dependency.
            .dependencies(dptree::deps![state_mutex])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
