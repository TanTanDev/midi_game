pub struct Gun {
    is_loaded: bool,
    latch_state: LatchState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatchState {
    Closed,
    Open,
}

pub enum ConsumeError {
    LatchWasOpen,
    NotLoaded,
}

impl Gun {
    pub fn new() -> Self {
        Self {
            is_loaded: false,
            latch_state: LatchState::Closed,
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.is_loaded && self.latch_state == LatchState::Closed
    }

    pub fn set_latch_state(&mut self, state: LatchState) {
        if self.latch_state == LatchState::Open && state == LatchState::Closed {
            self.is_loaded = true;
        }
        self.latch_state = state;
        println!("latch state: {:?}", self.latch_state);
    }

    pub fn try_consume(&mut self) -> Result<(), ConsumeError> {
        match self.latch_state {
            LatchState::Open => Err(ConsumeError::LatchWasOpen),
            LatchState::Closed => match self.is_loaded {
                true => {
                    self.is_loaded = false;
                    Ok(())
                }
                false => Err(ConsumeError::NotLoaded),
            },
        }
    }
}
