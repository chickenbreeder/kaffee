//! This module contains the necessary types for setting up a `kaffee` application.
//!
//! # Example
//!
//! The following example demonstrates the setup of an application:
//!
//! ```
//! use kaffee::prelude::*;
//!
//! struct GameState;
//!
//! impl EventHandler for GameState {
//!     fn init(&mut self, g: &mut GfxContext) {}
//!     fn input(&mut self) {}
//!     fn update(&mut self, dt: f32) {}
//!     fn redraw(&mut self, g: &mut GfxContext) {}
//! }
//!
//! fn main() {
//!     pollster::block_on(App::new(GameState {})).run();
//! }
//! ```
//!

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{config::Config, event::EventHandler, gfx::GfxContext, prelude::BatchExt};

/// A `kaffee` application.
pub struct App<H: 'static + EventHandler> {
    window: Window,
    event_loop: EventLoop<()>,
    event_handler: H,
    gfx_ctx: GfxContext,
}

impl<H: 'static + EventHandler> App<H> {
    pub async fn new(event_handler: H) -> Self {
        Self::with_config(event_handler, &Config::default()).await
    }

    pub async fn with_config(event_handler: H, config: &Config) -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
            .build(&event_loop)
            .expect("Failed to create window with given settings");

        let gfx_ctx = GfxContext::new(&window, config).await;

        Self {
            window,
            event_loop,
            event_handler,
            gfx_ctx,
        }
    }

    pub fn run(mut self) -> ! {
        self.event_handler.init(&mut self.gfx_ctx);

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::RedrawRequested(_) => {
                    self.event_handler.update(1.);
                    self.event_handler.redraw(&mut self.gfx_ctx);
                    self.gfx_ctx.end_frame();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                _ => (),
            }
        })
    }
}
