//! This module contains event-related types.

use crate::gfx::RenderContext;

/// This type allows interactions with the event loop.
pub trait EventHandler {
    fn update(&mut self, dt: f32);
    fn redraw(&mut self, r: &mut RenderContext);
}
