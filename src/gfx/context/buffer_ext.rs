use bytemuck::Pod;

use crate::gfx::{
    buffer::{Buffer, MutableBuffer},
    types::BufferUsages,
};

use super::GfxContext;

pub trait BufferExt {
    fn create_buffer<T: Pod>(&self, usage: BufferUsages, data: &[T]) -> Buffer<T>;

    fn create_buffer_mut<T: Pod>(&self, usage: BufferUsages, capacity: u64) -> MutableBuffer<T>;
}

pub(super) fn create_buffer<T: Pod>(
    device: &wgpu::Device,
    usage: BufferUsages,
    data: &[T],
) -> Buffer<T> {
    Buffer::from_data(device, usage, data)
}

pub(super) fn create_buffer_mut<T: Pod>(
    device: &wgpu::Device,
    usage: BufferUsages,
    capacity: u64,
) -> MutableBuffer<T> {
    MutableBuffer::with_capacity(device, usage, capacity)
}

impl BufferExt for GfxContext {
    fn create_buffer<T: Pod>(&self, usage: BufferUsages, data: &[T]) -> Buffer<T> {
        create_buffer(&self.device, usage, data)
    }

    fn create_buffer_mut<T: Pod>(&self, usage: BufferUsages, capacity: u64) -> MutableBuffer<T> {
        create_buffer_mut(&self.device, usage, capacity)
    }
}
