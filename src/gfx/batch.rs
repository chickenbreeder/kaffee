use std::mem;

use wgpu::{RenderPipeline, ShaderModule, TextureFormat};

use super::{
    buffer::{ImmutableBuffer, MutableBuffer},
    camera::Camera2D,
    color::Color,
};

const MAX_QUAD_COUNT: u64 = 200;
const MAX_VERTEX_COUNT: u64 = MAX_QUAD_COUNT * 4;
const MAX_INDEXES_COUNT: u64 = MAX_QUAD_COUNT * 6;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub(crate) struct Vertex {
    pub(crate) pos: [f32; 3],
    pub(crate) color: [f32; 3],
}

pub struct BatchPipeline {
    render_pipeline: RenderPipeline,
    vertex_buffer: MutableBuffer<Vertex>,
    index_buffer: ImmutableBuffer<u16>,
    camera_buffer: MutableBuffer<Camera2D>,
    camera_bind_group: wgpu::BindGroup,
    vertices: Vec<Vertex>,
    vertices_off: usize,
}

impl BatchPipeline {
    pub fn new(
        device: &wgpu::Device,
        supported_formats: &[TextureFormat],
        vertex_shader: ShaderModule,
        fragment_shader: ShaderModule,
        camera: &Camera2D,
    ) -> Self {
        let vertex_buffer: MutableBuffer<Vertex> =
            MutableBuffer::with_capacity(device, wgpu::BufferUsages::VERTEX, MAX_VERTEX_COUNT);

        let mut indices = [0u16; MAX_INDEXES_COUNT as usize];
        let mut offset = 0;

        for i in (0..MAX_INDEXES_COUNT as usize).step_by(6) {
            indices[i] = 0 + offset;
            indices[i + 1] = 1 + offset;
            indices[i + 2] = 2 + offset;
            indices[i + 3] = 2 + offset;
            indices[i + 4] = 3 + offset;
            indices[i + 5] = 0 + offset;
            offset += 4;
        }

        let index_buffer = ImmutableBuffer::from_data(device, wgpu::BufferUsages::INDEX, &indices);
        let camera_buffer =
            MutableBuffer::from_data(device, wgpu::BufferUsages::UNIFORM, &[*camera]);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &&camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.handle().as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: supported_formats[0],
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut vertices = Vec::with_capacity(MAX_VERTEX_COUNT as usize);
        unsafe {
            vertices.set_len(MAX_VERTEX_COUNT as usize);
        }

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            camera_buffer,
            camera_bind_group,
            vertices,
            vertices_off: 0,
        }
    }

    pub fn flush(&mut self, queue: &wgpu::Queue) {
        self.vertex_buffer.upload(queue, &self.vertices);
        self.vertices_off = 0;
    }

    pub(crate) fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

    pub(crate) fn vertex_buffer(&self) -> &MutableBuffer<Vertex> {
        &self.vertex_buffer
    }

    pub(crate) fn index_buffer(&self) -> &ImmutableBuffer<u16> {
        &self.index_buffer
    }

    pub(crate) fn camera_bind_group(&self) -> &wgpu::BindGroup {
        &self.camera_bind_group
    }

    pub(crate) fn push_quad(&mut self, x: f32, y: f32, color: Color) {
        self.vertices[self.vertices_off] = Vertex {
            pos: [x - 0.5, y + 0.5, 0.0],
            color: color.into(),
        };
        self.vertices[self.vertices_off + 1] = Vertex {
            pos: [x + 0.5, y + 0.5, 0.0],
            color: color.into(),
        };
        self.vertices[self.vertices_off + 2] = Vertex {
            pos: [x + 0.5, y - 0.5, 0.0],
            color: color.into(),
        };
        self.vertices[self.vertices_off + 3] = Vertex {
            pos: [x - 0.5, y - 0.5, 0.0],
            color: color.into(),
        };
        self.vertices_off += 4;
    }
}
