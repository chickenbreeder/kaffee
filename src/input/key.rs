#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up = winit::event::VirtualKeyCode::Up as u32,
    Down,
    Left,
    Right,
}
