use std::{mem, ops::Deref};

use bytemuck::Pod;
use wgpu::util::DeviceExt;

use super::types::BufferUsages;

pub struct Buffer<T: Pod> {
    buffer: wgpu::Buffer,
    cap: u64,
    len: u64,
    data: std::marker::PhantomData<T>,
}

impl<T: Pod> Buffer<T> {
    pub(super) fn from_data(device: &wgpu::Device, usage: BufferUsages, data: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage,
        });

        let len = data.len() as u64;

        Self {
            buffer,
            cap: len,
            len,
            data: std::marker::PhantomData,
        }
    }

    pub fn handle(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn cap(&self) -> u64 {
        self.cap
    }
}

/// The data of a [`MutableBuffer`] can be updated dynamically.
pub struct MutableBuffer<T: Pod>(Buffer<T>);

impl<T: Pod> Deref for MutableBuffer<T> {
    type Target = Buffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Pod> MutableBuffer<T> {
    pub fn with_capacity(device: &wgpu::Device, usage: BufferUsages, capacity: u64) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: mem::size_of::<T>() as u64 * capacity,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self(Buffer {
            buffer,
            cap: capacity,
            len: 0,
            data: std::marker::PhantomData,
        })
    }

    pub fn upload(&self, queue: &wgpu::Queue, data: &[T]) {
        let data = bytemuck::cast_slice(data);
        queue.write_buffer(&self.0.buffer, 0, data);
    }
}
