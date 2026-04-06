use gilrs::{Axis, Button, Gilrs};

/// Gamepad input system for USB controllers/joysticks
/// Supports Xbox 360/One, PlayStation, and generic USB gamepads
pub struct GamepadInput {
    gilrs: Gilrs,
}

impl std::fmt::Debug for GamepadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GamepadInput").finish()
    }
}

impl GamepadInput {
    pub fn new() -> Self {
        GamepadInput {
            gilrs: Gilrs::new().expect("Failed to initialize gamepad system"),
        }
    }

    /// Update gamepad state (call once per frame)
    pub fn update(&mut self) {
        // Process events and update state
        while let Some(_event) = self.gilrs.next_event() {
            // Events are automatically processed by gilrs
        }
    }

    /// Check if gamepad 0 (first connected controller) is present
    #[allow(dead_code)]
    pub fn is_connected(&self) -> bool {
        self.gilrs.gamepads().next().is_some()
    }

    /// Get left stick X axis value (-1.0 to 1.0)
    /// Returns 0.0 if no gamepad connected
    pub fn get_left_stick_x(&self, deadzone: f32) -> f32 {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            let value = gamepad.value(Axis::LeftStickX);
            if value.abs() > deadzone {
                value
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Get left stick Y axis value (-1.0 to 1.0)
    /// Returns 0.0 if no gamepad connected
    #[allow(dead_code)]
    pub fn get_left_stick_y(&self, deadzone: f32) -> f32 {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            let value = gamepad.value(Axis::LeftStickY);
            if value.abs() > deadzone {
                value
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Check if D-Pad Left is pressed
    pub fn is_dpad_left_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::DPadLeft)
        } else {
            false
        }
    }

    /// Check if D-Pad Right is pressed
    pub fn is_dpad_right_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::DPadRight)
        } else {
            false
        }
    }

    /// Check if South button (A on Xbox, X on PS) is pressed
    pub fn is_south_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::South)
        } else {
            false
        }
    }

    /// Check if Start button is pressed
    pub fn is_start_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::Start)
        } else {
            false
        }
    }

    /// Check if Select/Back button is pressed
    pub fn is_select_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::Select)
        } else {
            false
        }
    }

    /// Check if Left Shoulder button (LB on Xbox, L1 on PS) is pressed
    pub fn is_lb_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::LeftTrigger)
        } else {
            false
        }
    }

    /// Check if Right Shoulder button (RB on Xbox, R1 on PS) is pressed
    pub fn is_rb_pressed(&self) -> bool {
        if let Some((_id, gamepad)) = self.gilrs.gamepads().next() {
            gamepad.is_pressed(Button::RightTrigger)
        } else {
            false
        }
    }

    /// Get gamepad name/model if connected
    #[allow(dead_code)]
    pub fn get_gamepad_name(&self) -> Option<String> {
        self.gilrs
            .gamepads()
            .next()
            .map(|(_id, gp)| gp.name().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamepad_creation() {
        let _gamepad = GamepadInput::new();
        // Should not panic
    }
}
