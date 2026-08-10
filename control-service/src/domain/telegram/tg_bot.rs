pub enum InfoState {
    Start,
    GetSystemState,
    GetCurrentConfig,
    GetChosenInstrumentInfo { ticker: str },
}

pub enum CommandState {
    SetInstrument { ticker: str },
    SetMic,
    SetAmount { amount: u32 },
    Buy,
    Sell,
    ClosePosition,
}
