use glam::vec2;
use std::mem;

use super::{texture::Texture2D, MAX_INDEXES_COUNT, MAX_QUAD_COUNT, MAX_VERTEX_COUNT};
use crate::{
    gfx::{color::Color, Vertex},
    math::Rect,
};

pub struct BatchContext {
    vertices: Vec<Vertex>,
    vertices_off: usize,
    bind_group: wgpu::BindGroup,
    texture: Texture2D,
}

impl BatchContext {
    const DEFAULT_UV: Rect = Rect {
        min: vec2(0., 0.),
        max: vec2(1., 1.),
    };

    pub(crate) fn new(
        capacity: usize,
        texture: Texture2D,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(texture.sampler()),
                },
            ],
            label: None,
        });
        let mut vertices = Vec::with_capacity(capacity);
        unsafe {
            vertices.set_len(MAX_VERTEX_COUNT as usize);
        }

        Self {
            vertices,
            vertices_off: 0,
            bind_group,
            texture,
        }
    }

    /// Draws a rectangle with given position, dimension and color.
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.draw_texture_region(x, y, w, h, color, Self::DEFAULT_UV);
    }

    pub(crate) fn draw_texture_region(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        uv: Rect,
    ) {
        self.vertices[self.vertices_off] = Vertex {
            pos: [x + w, y + h, 0.0],
            color: color.into(),
            tex_coords: [uv.max.x, uv.max.y],
        };
        self.vertices[self.vertices_off + 1] = Vertex {
            pos: [x + w, y, 0.0],
            color: color.into(),
            tex_coords: [uv.max.x, uv.min.y],
        };
        self.vertices[self.vertices_off + 2] = Vertex {
            pos: [x, y, 0.0],
            color: color.into(),
            tex_coords: [uv.min.x, uv.min.y],
        };
        self.vertices[self.vertices_off + 3] = Vertex {
            pos: [x, y + h, 0.0],
            color: color.into(),
            tex_coords: [uv.min.x, uv.max.y],
        };
        self.vertices_off += 4;
    }

    pub(crate) fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub(crate) fn reset(&mut self) {
        self.vertices_off = 0;
    }

    pub(crate) fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
