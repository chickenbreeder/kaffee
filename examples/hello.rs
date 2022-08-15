use kaffee::prelude::*;

struct GameState {}

impl EventHandler for GameState {
    fn update(&mut self) {}

    fn fixed_update(&mut self) {}

    fn redraw(&mut self) {
        draw_quad(-0.5, 0.5, RED);
        draw_quad(0.5, -0.5, GREEN);
        draw_quad(0.5, 0.5, YELLOW);
        end_frame();
    }
}

fn main() {
    let settings = Settings::default();
    let state = GameState {};
    let app = pollster::block_on(App::new(&settings, state));
    app.run()
}
