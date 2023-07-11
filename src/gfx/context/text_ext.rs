use wgpu_glyph::{Section, Text};

use crate::prelude::Color;

use super::GfxContext;

pub trait TextExt {
    fn draw_text(&mut self, x: f32, y: f32, scale: f32, color: Color, text: &str);
}

impl TextExt for GfxContext {
    fn draw_text(&mut self, x: f32, y: f32, scale: f32, color: Color, text: &str) {
        self.glyph_brush.queue(Section {
            screen_position: (x, y),
            bounds: (1024 as f32, 768 as f32),
            text: vec![Text::new(text)
                .with_color([color.r, color.g, color.b, color.a])
                .with_scale(scale)],
            ..Section::default()
        });
    }
}
