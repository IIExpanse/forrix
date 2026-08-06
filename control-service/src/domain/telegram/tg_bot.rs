pub enum InfoState {
    Start,
    GetSystemState,
    GetCurrentConfig,
    GetChosenInstrumentInfo,
}

pub enum CommandState {
    SetInstrument,
    SetAmount,
    Buy,
    Sell,
    ClosePosition,
}
