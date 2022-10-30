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

    pub fn draw_quad(&mut self, x: f32, y: f32, color: Color) {
        self.vertices[self.vertices_off] = Vertex {
            pos: [x - 0.5, y + 0.5, 0.0],
            color: color.into(),
            tex_coords: [0., 1.],
        };
        self.vertices[self.vertices_off + 1] = Vertex {
            pos: [x + 0.5, y + 0.5, 0.0],
            color: color.into(),
            tex_coords: [1., 1.],
        };
        self.vertices[self.vertices_off + 2] = Vertex {
            pos: [x + 0.5, y - 0.5, 0.0],
            color: color.into(),
            tex_coords: [1., 0.],
        };
        self.vertices[self.vertices_off + 3] = Vertex {
            pos: [x - 0.5, y - 0.5, 0.0],
            color: color.into(),
            tex_coords: [0., 0.],
        };
        self.vertices_off += 4;
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
