use std::path::Path;

use crate::gfx::texture::Texture;

use super::GfxContext;

/// This extension trait enables texture creation for the [`GfxContext`].
pub trait TextureExt {
    fn create_texture<P: AsRef<Path>>(&self, path: P) -> Texture;
}

impl TextureExt for GfxContext {
    fn create_texture<P: AsRef<Path>>(&self, path: P) -> Texture {
        Texture::from_path(path, &self.device, &self.queue).expect("Failed to create texture")
    }
}
