pub enum SimpleState {
    Closed,
    Opened { direction: Direction },
}

pub enum Direction {
    Long,
    Short,
}
