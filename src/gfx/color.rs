#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub mod colors {

    use super::Color;

    pub const WHITE: Color = Color::new(1., 1., 1., 1.);
    pub const RED: Color = Color::new(1., 0., 0., 1.);
    pub const GREEN: Color = Color::new(0., 1., 0., 1.);
    pub const BLUE: Color = Color::new(0., 0., 1., 1.);
    pub const YELLOW: Color = Color::new(1., 1., 0., 1.);
}
