use crate::gfx::{types::{Pipeline, Vertex}, texture::TextureRef};

use super::{GfxContext, PipelineDescriptor};

pub(super) fn create_pipeline(device: &wgpu::Device, descriptor: &PipelineDescriptor, default_texture: &TextureRef) -> Pipeline {

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[default_texture.bind_group_layout()],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &descriptor.vertex_shader,
            entry_point: "main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &descriptor.fragment_shader,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: descriptor.texture_format,
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

    render_pipeline
}

pub trait PipelineExt {
    fn set_pipeline(&mut self, pipeline: &Pipeline);

    fn create_pipeline(&self, descriptor: &PipelineDescriptor, default_texture: &TextureRef) -> Pipeline;
}

impl PipelineExt for GfxContext {
    fn set_pipeline(&mut self, pipeline: &Pipeline) {
        unimplemented!()
    }

    fn create_pipeline(&self, descriptor: &PipelineDescriptor, default_texture: &TextureRef) -> Pipeline {
        create_pipeline(&self.device, descriptor, default_texture)
    }
}
