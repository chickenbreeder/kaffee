# kaffee

This is an attempt at designing a 2D graphics API, heavily inspired by [macroquad](https://github.com/not-fl3/macroquad), on top of [wgpu](https://github.com/gfx-rs/wgpu). You should probably not use this (yet).

## Example

```rust
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
```
