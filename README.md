# kaffee

[![Docs](https://docs.rs/kaffee/badge.svg)](https://docs.rs/kaffee/latest)

This is an attempt at designing a simple 2D graphics API on top of [wgpu](https://github.com/gfx-rs/wgpu), inspired by [macroquad](https://github.com/not-fl3/macroquad). You should probably not use this (yet).

## Example

```rust
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
        });
    }
}

fn main() {
    pollster::block_on(App::run(&Settings::default(), GameState {}))
}
```

## Goals

* Good documentation and examples
* Provide an easy to use API
* Support a wide range of platforms
* Decent performance

## License

Apache License, Version 2.0
