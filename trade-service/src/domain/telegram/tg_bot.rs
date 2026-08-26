use std::sync::{Arc, Mutex};

use teloxide::{
    Bot,
    dispatching::{Dispatcher, UpdateFilterExt},
    dptree,
    requests::Requester,
    types::{Message, Update},
    utils::command::BotCommands,
};
use tracing::{debug, error};

use crate::domain::trade::trade_engine::TradeState;

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

struct TgChat;

// todo refactor one-arg setter functions into generic
impl TgChat {
    async fn start_dispatcher(bot: Bot, state_mutex: Arc<Mutex<TradeState>>) {
        let handler = Update::filter_message().endpoint(
            |local_bot: Bot, msg: Message, state_mutex: Arc<Mutex<TradeState>>| async {
                let message = msg.text();
                if message.is_none() {
                    return Self::log_output("processing input", || {
                        Self::send_message(
                            local_bot,
                            msg,
                            "Text command is expected. Type /start to see command list.".to_owned(),
                        )
                    })
                    .await;
                }

                let parts: Vec<&str> = message.unwrap().split(" ").collect();
                if parts.is_empty() {
                    return Self::log_output("processing input", || {
                        Self::send_message(
                            local_bot,
                            msg,
                            "Message is empty. Type /start to see command list.".to_owned(),
                        )
                    })
                    .await;
                }

                let cmd = Command::parse(parts[0].trim(), "bot_username");
                if cmd.is_err() {
                    return Self::log_output("processing input", || {
                        Self::send_message(
                            local_bot,
                            msg,
                            "Command is invalid. Type /start to see command list.".to_owned(),
                        )
                    })
                    .await;
                }

                let res = match cmd.as_ref().unwrap() {
                    Command::Start => {
                        Self::send_message(local_bot, msg, Command::descriptions().to_string())
                            .await
                    }
                    Command::GetSystemHealth => todo!(),
                    Command::GetCurrentConfig => {
                        Self::get_current_config(local_bot, msg, state_mutex).await
                    }
                    Command::GetChosenInstrumentInfo { ticker } => todo!(),
                    Command::SetInstrument { ticker } => {
                        Self::set_instrument(local_bot, msg, state_mutex, ticker).await
                    }
                    Command::SetMic { mic } => {
                        Self::set_mic(local_bot, msg, state_mutex, mic).await
                    }
                    Command::SetAmount { amount } => {
                        Self::set_amount(local_bot, msg, state_mutex, amount).await
                    }
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

    async fn send_message(bot: Bot, msg: Message, text: String) -> HandlerResult {
        bot.send_message(msg.chat.id, text).await?;
        Ok(())
    }

    async fn get_current_config(
        bot: Bot,
        msg: Message,
        state_mutex: Arc<Mutex<TradeState>>,
    ) -> HandlerResult {
        let config;
        {
            config = state_mutex.lock().unwrap().clone();
        }

        bot.send_message(msg.chat.id, format!("Current config: {:#?}", config))
            .await?;

        Ok(())
    }

    async fn set_instrument(
        bot: Bot,
        msg: Message,
        state_mutex: Arc<Mutex<TradeState>>,
        ticker: &str,
    ) -> HandlerResult {
        {
            state_mutex.lock().unwrap().ticker = Some(ticker.to_owned());
        }

        bot.send_message(msg.chat.id, format!("Saved: ticker = {}", ticker))
            .await?;

        Ok(())
    }

    async fn set_mic(
        bot: Bot,
        msg: Message,
        state_mutex: Arc<Mutex<TradeState>>,
        mic: &str,
    ) -> HandlerResult {
        {
            state_mutex.lock().unwrap().mic = Some(mic.to_owned());
        }

        bot.send_message(msg.chat.id, format!("Saved: mic = {}", mic))
            .await?;

        Ok(())
    }

    async fn set_amount(
        bot: Bot,
        msg: Message,
        state_mutex: Arc<Mutex<TradeState>>,
        amount: &u32,
    ) -> HandlerResult {
        {
            state_mutex.lock().unwrap().amount = Some(*amount);
        }

        bot.send_message(msg.chat.id, format!("Saved: amount = {}", amount))
            .await?;

        Ok(())
    }

    async fn log_output<F>(action_description: &str, func: F) -> HandlerResult
    where
        F: AsyncFnOnce() -> HandlerResult,
    {
        let res = func().await;
        if let Err(err) = res {
            error!("Error while {}: {}", action_description, err);
            return Err(err);
        };
        debug!("Successfully processed command {}", action_description);
        res
    }
}
