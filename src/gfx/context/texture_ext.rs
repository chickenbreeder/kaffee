use std::path::Path;

use crate::gfx::texture::{FilterMode, Texture};

use super::GfxContext;

/// This extension trait enables texture creation for the [`GfxContext`].
pub trait TextureExt {
    fn create_texture<P: AsRef<Path>>(&self, path: P, filter_mode: FilterMode) -> Texture;
}

impl TextureExt for GfxContext {
    fn create_texture<P: AsRef<Path>>(&self, path: P, filter_mode: FilterMode) -> Texture {
        let texture = Texture::from_path(path, &self.device, &self.queue, filter_mode)
            .expect("Failed to create texture");
        todo!()
    }
}
