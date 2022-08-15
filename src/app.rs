//! The entry point for every application

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::gfx::RenderContext;

pub trait EventHandler {
    fn update(&mut self);
    fn fixed_update(&mut self);
    fn redraw(&mut self);
}

#[derive(Debug)]
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

pub struct App<H: EventHandler + 'static> {
    event_loop: EventLoop<()>,
    window: Window,
    event_handler: H,
}

impl<H: EventHandler + 'static> App<H> {
    pub async fn new(s: &Settings, event_handler: H) -> Self {
        env_logger::init();
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(&s.title)
            .with_inner_size(LogicalSize::new(s.width, s.height))
            .with_resizable(s.resizable)
            .build(&event_loop)
            .expect("Failed to create window with given settings");

        RenderContext::from_window(&window).await;

        Self {
            event_loop,
            window,
            event_handler,
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::RedrawRequested(_) => {
                    self.event_handler.redraw();
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
