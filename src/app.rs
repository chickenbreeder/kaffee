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
//!     fn update(&mut self, dt: f32) {}
//!     fn redraw(&mut self, r: &mut RenderContext) {}
//! }
//!
//! fn main() {
//!     pollster::block_on(App::run(
//!         &Settings::default(),
//!         GameState {}
//!     ))
//! }
//! ```
//!

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::gfx::RenderContext;

#[derive(Debug, Clone)]
pub struct Settings {
    pub(crate) title: String,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) resizable: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: "kaffee".into(),
            width: 1024,
            height: 768,
            resizable: false,
        }
    }
}

pub struct App;

pub trait EventHandler {
    fn update(&mut self, dt: f32);
    fn redraw(&mut self, r: &mut RenderContext);
}

impl App {
    /// The entry point for every application built with `kaffee`.
    pub async fn run<H>(settings: &Settings, mut event_handler: H) -> !
    where
        H: 'static + EventHandler,
    {
        env_logger::init();
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(&settings.title)
            .with_inner_size(LogicalSize::new(settings.width, settings.height))
            .with_resizable(settings.resizable)
            .build(&event_loop)
            .expect("Failed to create window with given settings");

        let mut render_context = RenderContext::new(&window).await;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::RedrawRequested(_) => {
                    event_handler.update(1.);
                    event_handler.redraw(&mut render_context);
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
