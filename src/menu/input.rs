use gilrs::{Button, EventType, GamepadId, Gilrs};
use quicksilver::{
    input::{ButtonState, Key},
    lifecycle::Window,
    Result,
};

pub struct Input {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl Input {
    pub fn new() -> Result<Self> {
        Ok(Input {
            gilrs: Gilrs::new()?,
            active_gamepad: None,
        })
    }

    /// Poll all possible input sources.
    pub fn poll(&mut self, window: &mut Window) -> (bool, bool) {
        let mut quit = false;
        let mut start: bool = false;

        // Examine new gamepad events using GilRs directly as Quicksilver doesn't see some of the
        // buttons.
        while let Some(event) = self.gilrs.next_event() {
            if self.active_gamepad.is_none() {
                // If we don't have an active gamepad yet, then we do now.
                self.active_gamepad = Some(event.id);
            }

            // Check the gamepad for edge-triggered scenarios such as a button being pressed or
            // released in this turn.
            match event.event {
                // Quitting and starting are edge-triggered.
                EventType::ButtonReleased(Button::Start, _) => quit = true,
                EventType::ButtonPressed(Button::South, _) => start = true,
                _ => (),
            };
        }

        // Check the keyboard for edge-triggered events. Quitting and starting are edge-triggered.
        quit = quit || window.keyboard()[Key::Escape] == ButtonState::Released;
        start = start || window.keyboard()[Key::Space] == ButtonState::Pressed;

        (quit, start)
    }
}
