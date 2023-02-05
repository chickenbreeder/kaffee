use kaffee::prelude::*;

struct GameState {
    custom_batch_ctx: Option<BatchContext>,
}

impl EventHandler for GameState {
    fn init(&mut self, r: &mut RenderContext) {
        let texture = r.load_texture("./res/textures/atlas.png").unwrap();
        self.custom_batch_ctx = Some(r.create_batch(texture));
    }

    fn update(&mut self, dt: f32) {}

    fn redraw(&mut self, r: &mut RenderContext) {
        r.draw_batch_ex(self.custom_batch_ctx.as_mut().unwrap(), |b| {
            b.draw_rect(50., 50., 300., 300., GREEN);
            b.draw_rect(150., 150., 300., 300., RED);
            b.draw_rect(250., 250., 300., 300., WHITE);
        });
    }
}

fn main() {
    pollster::block_on(App::run(
        &Settings::default(),
        GameState {
            custom_batch_ctx: None,
        },
    ))
}
