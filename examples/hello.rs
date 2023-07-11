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
        g.draw_quad(-0.5, -0.5, 1., Color::RED);
    }
}

fn main() {
    pollster::block_on(App::new(GameState {})).run();
}
