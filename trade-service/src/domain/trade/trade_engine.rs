pub enum SimpleState {
    Closed,
    Opened { direction: Direction },
}

pub enum Direction {
    Long,
    Short,
}

#[derive(Debug, Clone)]
pub struct TradeState {
    pub ticker: Option<String>,
    pub mic: Option<String>,
    pub amount: Option<u32>,
}

struct TradeEngine {
    state: Option<TradeState>,
    position_opened: bool,
}

impl TradeEngine {
    fn new() -> Self {
        TradeEngine {
            state: None,
            position_opened: false,
        }
    }

    fn set_state(&mut self, state: TradeState) {
        self.state = Some(state)
    }

    fn buy(&mut self) {
        todo!()
    }

    fn sell(&mut self) {
        todo!()
    }
}
