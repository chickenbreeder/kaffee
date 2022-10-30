use kaffee::{gfx::context::RenderContext, prelude::*};

struct GameState {}

impl EventHandler for GameState {
    fn update(&mut self, dt: f32) {}

    fn redraw(&mut self, ctx: &mut RenderContext) {
        ctx.draw_quad(0.5, 0.5, RED);
        ctx.draw_quad(1.5, 1.5, GREEN);
        ctx.draw_quad(2.5, 2.5, BLUE);
        ctx.draw_quad(3.5, 3.5, YELLOW);
        ctx.draw_quad(4.5, 4.5, PINK);
        ctx.draw_quad(5.5, 5.5, WHITE);

        ctx.draw_atlas_index(6.5, 6.5, WHITE, 0);
        ctx.draw_atlas_index(7.5, 6.5, WHITE, 1);
        ctx.draw_atlas_index(6.5, 7.5, WHITE, 2);
        ctx.draw_atlas_index(7.5, 7.5, WHITE, 3);
        ctx.end_frame();
    }
}

fn main() {
    let settings = Settings::default();
    let state = GameState {};
    pollster::block_on(App::run(&settings, state))
}
