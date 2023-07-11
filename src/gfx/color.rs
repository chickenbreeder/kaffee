#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color::new(0., 0., 0., 1.);
    pub const WHITE: Color = Color::new(1., 1., 1., 1.);

    pub const RED: Color = Color::new(1., 0.2, 0.2, 1.);
    pub const GREEN: Color = Color::new(0.2, 0.9, 0.2, 1.);
    pub const BLUE: Color = Color::new(0., 0.2, 1., 1.);
    pub const YELLOW: Color = Color::new(1., 0.86, 0., 0.5);
    pub const PINK: Color = Color::new(1., 0., 1., 0.5);

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

#[doc(hidden)]
impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}
