# kaffee

This is an attempt at designing a simple 2D graphics API on top of [wgpu](https://github.com/gfx-rs/wgpu), inspired by [macroquad](https://github.com/not-fl3/macroquad). You should probably not use this (yet).

## Example

```rust
use kaffee::prelude::*;

struct GameState {}

impl EventHandler for GameState {
    fn init(&mut self, ctx: &mut GraphicsContext) {}

    fn update(&mut self, dt: f32) {}

    fn redraw(&mut self, ctx: &mut GraphicsContext) {
        ctx.draw_quad(0.5, 0.5, RED);
        ctx.draw_quad(1.5, 1.5, GREEN);
        ctx.draw_quad(2.5, 2.5, BLUE);
        ctx.draw_quad(3.5, 3.5, YELLOW);
        ctx.draw_quad(4.5, 4.5, PINK);
        ctx.draw_quad(5.5, 5.5, WHITE);
        ctx.end_frame();
    }
}

fn main() {
    let settings = Settings::default();
    let state = GameState {};
    pollster::block_on(App::run(&settings, state))
}
```
