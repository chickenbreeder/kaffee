use kaffee::prelude::*;

struct GameState {}

impl EventHandler for GameState {
    fn update(&mut self) {}

    fn fixed_update(&mut self) {}

    fn redraw(&mut self) {
        draw_quad(0.5, 0.5, RED);
        draw_quad(1.5, 1.5, GREEN);
        draw_quad(2.5, 2.5, BLUE);
        draw_quad(3.5, 3.5, YELLOW);
        draw_quad(4.5, 4.5, PINK);
        end_frame();
    }
}

fn main() {
    let settings = Settings::default();
    let state = GameState {};
    let app = pollster::block_on(App::new(&settings, state));
    app.run()
}
