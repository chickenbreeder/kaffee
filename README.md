# kaffee

This is an attempt at designing a simple 2D graphics API on top of [wgpu](https://github.com/gfx-rs/wgpu), inspired by [macroquad](https://github.com/not-fl3/macroquad). You should probably not use this (yet).

## Example

```rust
use kaffee::prelude::*;

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
```
