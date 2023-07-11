//! Re-exports the most common types.

pub use crate::{
    app::App,
    config::Config,
    error::ErrorKind,
    event::EventHandler,
    gfx::{BatchExt, Color, FilterMode, GfxContext, PipelineExt, TextExt, TextureExt},
    input::InputEvent,
};
