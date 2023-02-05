use kaffee::prelude::*;

struct GameState;

impl EventHandler for GameState {
    fn init(&mut self, r: &mut RenderContext) {}

    fn update(&mut self, dt: f32) {}

    fn redraw(&mut self, r: &mut RenderContext) {
        r.draw_batch(|b| {
            b.draw_rect(50., 50., 300., 300., GREEN);
            b.draw_rect(150., 150., 300., 300., RED);
            b.draw_rect(250., 250., 300., 300., WHITE);
            b.draw_rect(350., 350., 100., 100., BLACK);
        });
    }
}

fn main() {
    pollster::block_on(App::run(&Settings::default(), GameState {}))
}
