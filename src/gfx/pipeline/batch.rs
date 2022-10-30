use std::mem;

use wgpu::{RenderPipeline, ShaderModule, TextureFormat};

use crate::{
    gfx::{
        batch_context::BatchContext,
        buffer::{ImmutableBuffer, MutableBuffer},
        camera::Camera2D,
        color::Color,
        texture::Texture2D,
        texture_atlas::TextureAtlas,
        Vertex, MAX_INDEXES_COUNT, MAX_QUAD_COUNT, MAX_VERTEX_COUNT,
    },
    math::Rect,
};

pub struct BatchPipeline {
    render_pipeline: RenderPipeline,
    vertex_buffer: MutableBuffer<Vertex>,
    index_buffer: ImmutableBuffer<u16>,
    camera_buffer: MutableBuffer<Camera2D>,
    camera_bind_group: wgpu::BindGroup,
    diffuse_bind_group: wgpu::BindGroup,
}

impl BatchPipeline {
    pub(crate) fn new(
        device: &wgpu::Device,
        supported_formats: &[TextureFormat],
        vertex_shader: ShaderModule,
        fragment_shader: ShaderModule,
        camera: &Camera2D,
        texture_atlas: &TextureAtlas,
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

        let diffuse_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: None,
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &&camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.handle().as_entire_binding(),
            }],
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &diffuse_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_atlas.texture().view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(texture_atlas.texture().sampler()),
                },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout, &diffuse_bind_group_layout],
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
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
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

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            camera_buffer,
            camera_bind_group,
            diffuse_bind_group,
        }
    }

    pub fn flush(&mut self, queue: &wgpu::Queue, batch_context: &BatchContext) {
        self.vertex_buffer.upload(queue, batch_context.vertices());
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

    pub(crate) fn diffuse_bind_group(&self) -> &wgpu::BindGroup {
        &&self.diffuse_bind_group
    }
}
