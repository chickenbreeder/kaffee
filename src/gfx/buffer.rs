use std::{mem, ops::Deref};

use wgpu::util::DeviceExt;

/// An [`ImmutableBuffer`] should be used for read-only data.
pub struct ImmutableBuffer<T> {
    buffer: wgpu::Buffer,
    cap: u64,
    len: u64,
    data: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> ImmutableBuffer<T> {
    /// Creates a new instance of [`ImmutableBuffer`] with the given usage flags.
    /// An example might look like this:
    /// ```rust
    /// #[repr(C)]
    /// #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
    /// pub(crate) struct Vertex {
    ///     pub(crate) pos: [f32; 3],
    /// }
    ///
    /// fn foo() {
    ///     let vertices: Vec<Vertex> = vec![];
    ///     ImmutableBuffer::from_data(&device, wgpu::BufferUsages::VERTEX, &vertices);
    /// }
    /// ```
    pub fn from_data(device: &wgpu::Device, usage: wgpu::BufferUsages, data: &[T]) -> Self {
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
pub struct MutableBuffer<T>(ImmutableBuffer<T>);

impl<T: bytemuck::Pod> Deref for MutableBuffer<T> {
    type Target = ImmutableBuffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: bytemuck::Pod> MutableBuffer<T> {
    pub fn with_capacity(device: &wgpu::Device, usage: wgpu::BufferUsages, cap: u64) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: mem::size_of::<T>() as u64 * cap,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self(ImmutableBuffer {
            buffer,
            cap,
            len: 0,
            data: std::marker::PhantomData,
        })
    }

    pub fn upload(&self, queue: &wgpu::Queue, data: &[T]) {
        let data = bytemuck::cast_slice(data);
        queue.write_buffer(&self.0.buffer, 0, data);
    }
}
