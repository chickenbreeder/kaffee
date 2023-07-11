use winit::event::{ElementState, VirtualKeyCode};

mod key;

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub state: winit::event::ElementState,
    pub key: winit::event::VirtualKeyCode,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Key(KeyEvent),
}

impl InputEvent {
    pub fn is_key_pressed(self, key: VirtualKeyCode) -> bool {
        self.is_key_state(ElementState::Pressed, key)
    }

    pub fn is_key_released(self, key: VirtualKeyCode) -> bool {
        self.is_key_state(ElementState::Released, key)
    }

    fn is_key_state(self, state: ElementState, key: VirtualKeyCode) -> bool {
        match self {
            InputEvent::Key(KeyEvent {
                state: _state,
                key: _key,
            }) => {
                if state == _state && key == _key {
                    return true;
                }
            }
        }
        false
    }
}
