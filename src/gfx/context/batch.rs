use std::ops::{Index, IndexMut};

use crate::gfx::{
    buffer::{Buffer, MutableBuffer},
    types::{BufferUsages, Vertex},
};

use super::buffer_ext::{create_buffer, create_buffer_mut};

pub(super) struct Batch<const QUAD_COUNT: usize> {
    vertices: Vec<Vertex>,
    vertices_off: usize,
    index_buffer: Buffer<u16>,
    vertex_buffer: MutableBuffer<Vertex>,
}

impl<const QUAD_COUNT: usize> Batch<QUAD_COUNT> {
    const VERTEX_COUNT: usize = QUAD_COUNT * 4;
    const INDEX_COUNT: usize = QUAD_COUNT * 6;

    pub fn new(device: &wgpu::Device) -> Self {
        let mut vertices = Vec::with_capacity(Self::VERTEX_COUNT);
        unsafe {
            vertices.set_len(Self::VERTEX_COUNT);
        }

        let mut indices = Vec::with_capacity(Self::INDEX_COUNT);
        unsafe {
            indices.set_len(Self::INDEX_COUNT as usize);
        }

        let mut offset = 0;

        for i in (0..Self::INDEX_COUNT as usize).step_by(6) {
            indices[i] = 0 + offset;
            indices[i + 1] = 1 + offset;
            indices[i + 2] = 2 + offset;
            indices[i + 3] = 2 + offset;
            indices[i + 4] = 3 + offset;
            indices[i + 5] = 0 + offset;
            offset += 4;
        }

        let index_buffer = create_buffer(&device, BufferUsages::INDEX, &indices);
        let vertex_buffer =
            create_buffer_mut(&device, BufferUsages::VERTEX, Self::VERTEX_COUNT as u64);

        Self {
            vertices,
            vertices_off: 0,
            index_buffer,
            vertex_buffer,
        }
    }

    pub fn offset(&self) -> usize {
        self.vertices_off
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.vertices_off = offset;
    }

    pub fn vertex_buffer(&self) -> &MutableBuffer<Vertex> {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer<u16> {
        &self.index_buffer
    }

    pub fn flush(&mut self, queue: &wgpu::Queue) {
        self.vertex_buffer.upload(queue, &self.vertices);
        self.vertices_off = 0;
    }
}

impl<const QUAD_COUNT: usize> Index<usize> for Batch<QUAD_COUNT> {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl<const QUAD_COUNT: usize> IndexMut<usize> for Batch<QUAD_COUNT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}
