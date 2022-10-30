use kaffee::{gfx::render::RenderContext, prelude::*};

struct GameState;

impl EventHandler for GameState {
    fn update(&mut self, dt: f32) {}

    fn redraw(&mut self, r: &mut RenderContext) {
        r.draw_batch(|b| {
            b.draw_quad(0.5, 0.5, RED);
            b.draw_quad(6.5, 6.5, GREEN);
            b.draw_quad(8.5, 8.5, BLUE);
        });
        r.end_frame();
    }
}

fn main() {
    let settings = Settings::default();
    let state = GameState {};
    pollster::block_on(App::run(&settings, state))
}
