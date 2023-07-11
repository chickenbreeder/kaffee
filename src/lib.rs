//! # kaffee - API Documentation
//!
//! kaffee is a simple 2D framework, built on top of [wgpu].
//!
//! [wgpu]: https://github.com/gfx-rs/wgpu/
pub mod app;
pub mod error;
pub mod event;
pub mod prelude;

mod config;
mod fs;
mod gfx;
mod input;
mod math;
