use kaffee::prelude::*;

struct GameState;

impl EventHandler for GameState {
    fn init(&mut self, _: &mut GfxContext) -> Result<(), ErrorKind> {
        Ok(())
    }

    fn input(&mut self, _: InputEvent) {}

    fn update(&mut self, _: f32) {}

    fn redraw(&mut self, g: &mut GfxContext) {
        g.clear_color(Color::BLACK);
        g.draw_text(35., 35., 40., Color::WHITE, "hello");
        g.draw_text(35., 50., 80., Color::GREEN, "world");
        g.draw_quad(-1., -1., 1., Color::RED);
        g.draw_quad(0., 0., 1., Color::BLUE);
    }
}

fn main() {
    pollster::block_on(App::new(GameState {})).run();
}
