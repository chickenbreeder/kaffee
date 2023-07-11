use std::path::Path;

use crate::{
    error::ErrorKind,
    gfx::texture::{FilterMode, Texture, TextureRef},
};

use super::GfxContext;

/// This extension trait enables texture creation for the [`GfxContext`].
pub trait TextureExt {
    /// Loads and creates a new texture from the specified path.
    fn create_texture<P: AsRef<Path>>(
        &self,
        path: P,
        filter_mode: FilterMode,
    ) -> Result<TextureRef, ErrorKind>;
}

impl TextureExt for GfxContext {
    fn create_texture<P: AsRef<Path>>(
        &self,
        path: P,
        filter_mode: FilterMode,
    ) -> Result<TextureRef, ErrorKind> {
        Texture::from_path(path, &self.device, &self.queue, filter_mode)
    }
}
