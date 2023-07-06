//! This module contains event-related types.

use crate::gfx::GfxContext;

/// This type allows interactions with the event loop.
pub trait EventHandler {
    fn init(&mut self, g: &mut GfxContext);

    fn input(&mut self);

    fn update(&mut self, dt: f32);

    fn redraw(&mut self, g: &mut GfxContext);
}
