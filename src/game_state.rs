use quicksilver::{lifecycle::Window, Result};

/// The action to be taken after a game state has been updated.
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

/// A game state (or screen) such as Playing, Paused, Loading, etc.
pub trait GameState {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        Action::Continue.into()
    }

    fn draw(&mut self, _window: &mut Window, _alpha: f64) -> Result<()> {
        Ok(())
    }
}
