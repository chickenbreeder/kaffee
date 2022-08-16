use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}
