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
    pub fn poll(&mut self, window: &mut Window) -> (bool, bool, bool, f32, f32) {
        let mut quit = false;
        let mut right_pressed = false;
        let mut left_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut fire: bool = false;
        let mut bomb: bool = false;

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
                // Quitting, firing and bombing are edge-triggered.
                EventType::ButtonReleased(Button::Start, _) => quit = true,
                EventType::ButtonPressed(Button::East, _) => fire = true,
                EventType::ButtonPressed(Button::South, _) => bomb = true,
                _ => (),
            };
        }

        // Check the gamepad for level-triggered events, such as button being held down. All
        // movement is level-triggered.
        if let Some(id) = self.active_gamepad {
            let gamepad = self.gilrs.gamepad(id);
            left_pressed = left_pressed || gamepad.is_pressed(Button::DPadLeft);
            right_pressed = right_pressed || gamepad.is_pressed(Button::DPadRight);
            up_pressed = up_pressed || gamepad.is_pressed(Button::DPadUp);
            down_pressed = down_pressed || gamepad.is_pressed(Button::DPadDown);
        }

        // Check the keyboard for edge-triggered events such as quitting, firing and bombing.
        quit = quit || window.keyboard()[Key::Escape] == ButtonState::Released;
        fire = fire
            || window.keyboard()[Key::Return] == ButtonState::Pressed
            || window.keyboard()[Key::P] == ButtonState::Pressed;
        bomb = bomb
            || window.keyboard()[Key::Space] == ButtonState::Pressed
            || window.keyboard()[Key::O] == ButtonState::Pressed;

        // Check the keyboard for level-triggered events. All movement is level-triggered.
        left_pressed = left_pressed
            || window.keyboard()[Key::Left].is_down()
            || window.keyboard()[Key::A].is_down();
        right_pressed = right_pressed
            || window.keyboard()[Key::Right].is_down()
            || window.keyboard()[Key::D].is_down();
        up_pressed = up_pressed
            || window.keyboard()[Key::Up].is_down()
            || window.keyboard()[Key::W].is_down();
        down_pressed = down_pressed
            || window.keyboard()[Key::Down].is_down()
            || window.keyboard()[Key::S].is_down();

        let dx = match (left_pressed, right_pressed) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        let dy = match (up_pressed, down_pressed) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        (quit, fire, bomb, dx, dy)
    }
}
