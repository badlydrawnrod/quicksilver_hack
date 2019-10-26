use quicksilver::{lifecycle::Window, Result};

pub enum Action {
    /// Stop the entire state machine (or game).
    Quit,
    /// Continue in the current state.
    Continue,
    /// Switch to the desired state.
    Transition(Box<dyn GameState>),
}

impl From<Action> for Result<Action> {
    fn from(r: Action) -> Self {
        Ok(r)
    }
}

pub trait GameState {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        Action::Continue.into()
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
}
