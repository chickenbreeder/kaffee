use glam::vec2;
use std::mem;

use super::{MAX_INDEXES_COUNT, MAX_QUAD_COUNT, MAX_VERTEX_COUNT};
use crate::{
    gfx::{color::Color, Vertex},
    math::Rect,
};

pub struct BatchContext {
    vertices: Vec<Vertex>,
    vertices_off: usize,
}

impl BatchContext {
    /// Creates a `BatchContext` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut vertices = Vec::with_capacity(capacity);
        unsafe {
            vertices.set_len(MAX_VERTEX_COUNT as usize);
        }

        Self {
            vertices,
            vertices_off: 0,
        }
    }

    /// Draws a quad with given position and color.
    pub fn draw_quad(&mut self, x: f32, y: f32, color: Color) {
        let uv = Rect {
            min: vec2(0., 0.),
            max: vec2(1., 1.),
        };
        self.draw_texture_region(x, y, color, uv);
    }

    pub(crate) fn draw_texture_region(&mut self, x: f32, y: f32, color: Color, uv: Rect) {
        self.vertices[self.vertices_off] = Vertex {
            pos: [x - 0.5, y + 0.5, 0.0],
            color: color.into(),
            tex_coords: [uv.min.x, uv.max.y],
        };
        self.vertices[self.vertices_off + 1] = Vertex {
            pos: [x + 0.5, y + 0.5, 0.0],
            color: color.into(),
            tex_coords: [uv.max.x, uv.max.y],
        };
        self.vertices[self.vertices_off + 2] = Vertex {
            pos: [x + 0.5, y - 0.5, 0.0],
            color: color.into(),
            tex_coords: [uv.max.x, uv.min.y],
        };
        self.vertices[self.vertices_off + 3] = Vertex {
            pos: [x - 0.5, y - 0.5, 0.0],
            color: color.into(),
            tex_coords: [uv.min.x, uv.min.y],
        };
        self.vertices_off += 4;
    }

    pub(crate) fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub(crate) fn reset(&mut self) {
        self.vertices_off = 0;
    }
}
