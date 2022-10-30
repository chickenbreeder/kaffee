//! Graphics-related code

pub mod batch_context;
pub mod buffer;
pub mod camera;
pub mod color;
pub mod render;
pub mod types;

mod pipeline;
mod texture;
mod texture_atlas;

pub use render::RenderContext;

const MAX_QUAD_COUNT: u64 = 200;
const MAX_VERTEX_COUNT: u64 = MAX_QUAD_COUNT * 4;
const MAX_INDEXES_COUNT: u64 = MAX_QUAD_COUNT * 6;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub(crate) struct Vertex {
    pub(crate) pos: [f32; 3],
    pub(crate) color: [f32; 3],
    pub(crate) tex_coords: [f32; 2],
}
