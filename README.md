# kaffee

[![Docs](https://docs.rs/kaffee/badge.svg)](https://docs.rs/kaffee/latest)
[![crates.io](https://img.shields.io/crates/v/kaffee.svg)](https://crates.io/crates/kaffee)

This is an attempt at designing a simple 2D graphics API on top of [wgpu](https://github.com/gfx-rs/wgpu), inspired by [macroquad](https://github.com/not-fl3/macroquad). You should probably not use this (yet).

## Example

```rust
use kaffee::prelude::*;

struct GameState;

impl EventHandler for GameState {
    fn init(&mut self, _: &mut GfxContext) {}

    fn input(&mut self) {}

    fn update(&mut self, _: f32) {}

    fn redraw(&mut self, g: &mut GfxContext) {
        g.clear_color(Color::BLACK);
        g.draw_quad(-0.5, -0.5, 1., Color::RED);
    }
}

fn main() {
    pollster::block_on(App::new(GameState {})).run();
}
```

## Goals

* Good documentation and examples
* Provide an easy to use API
* Support a wide range of platforms
* Decent performance

## License

Apache License, Version 2.0
