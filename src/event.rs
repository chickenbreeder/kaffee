//! This module contains event-related types.

use crate::{error::ErrorKind, gfx::GfxContext, input::InputEvent};

/// This type allows interactions with the event loop.
pub trait EventHandler {
    fn init(&mut self, g: &mut GfxContext) -> Result<(), ErrorKind>;

    fn input(&mut self, event: InputEvent);

    fn update(&mut self, dt: f32);

    fn redraw(&mut self, g: &mut GfxContext);
}
