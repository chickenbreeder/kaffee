use crate::gfx::types::Shader;

pub struct PipelineDescriptor {
    pub vertex_shader: Shader,
    pub fragment_shader: Shader,
    pub texture_format: wgpu::TextureFormat,
}
