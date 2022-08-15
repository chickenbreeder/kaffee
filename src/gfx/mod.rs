//! Graphics-related code

pub mod batch;
pub mod buffer;
pub mod color;
pub mod context;

pub use context::RenderContext;

use self::color::Color;

static mut RENDER_CONTEXT: Option<RenderContext> = None;

fn get_render_context() -> &'static mut RenderContext {
    unsafe {
        if let Some(render_context) = &mut RENDER_CONTEXT {
            return render_context;
        }
        panic!("");
    }
}

pub fn draw_quad(x: f32, y: f32, color: Color) {
    let render_context = get_render_context();
    render_context.draw_quad(x, y, color);
}

pub fn end_frame() {
    let render_context = get_render_context();
    render_context.end_frame();
}
