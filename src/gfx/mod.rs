//! Graphics-related code

pub mod buffer;
pub mod camera;
pub mod color;
pub mod context;

mod pipeline;

pub use context::RenderContext;

use self::color::Color;
