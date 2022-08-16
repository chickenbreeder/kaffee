use glam::{vec2, Vec2};

use crate::{error::ErrorKind, math::Rect};

use super::texture::Texture2D;

pub(crate) struct TextureAtlas {
    texture: Texture2D,
    cols: u16,
    rows: u16,
    regions: Vec<Rect>,
}

impl TextureAtlas {
    pub(crate) fn from_path<P: AsRef<std::path::Path>>(
        path: P,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cols: u16,
        rows: u16,
        tile_size: Vec2,
    ) -> Result<Self, ErrorKind> {
        let texture = Texture2D::from_path(path, device, queue)?;
        let width = texture.width() as f32;
        let height = texture.height() as f32;
        let mut regions = Vec::with_capacity((cols * rows) as usize);

        let mut x = 0.;
        let mut y = 0.;

        for _ in 0..rows {
            x = 0.;

            for _ in 0..cols {
                let min_x = x / width;
                let min_y = y / height;
                let max_x = (x + tile_size.x) / width;
                let max_y = (y + tile_size.y) / height;

                let region = Rect {
                    min: vec2(min_x, min_y),
                    max: vec2(max_x, max_y),
                };

                regions.push(region);

                x += tile_size.x;
            }

            y += tile_size.y;
        }

        Ok(Self {
            texture,
            cols,
            rows,
            regions,
        })
    }

    pub(crate) fn texture(&self) -> &Texture2D {
        &self.texture
    }

    pub(crate) fn get_region(&self, idx: u16) -> Rect {
        self.regions[idx as usize]
    }
}
