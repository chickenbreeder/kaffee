use std::borrow::Cow;

pub type Shader = wgpu::ShaderModule;
pub type Pipeline = wgpu::RenderPipeline;
pub type BufferUsages = wgpu::BufferUsages;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex = naga::ShaderStage::Vertex as isize,
    Fragment = naga::ShaderStage::Fragment as isize,
    Compute = naga::ShaderStage::Compute as isize,
}

impl Into<naga::ShaderStage> for ShaderStage {
    fn into(self) -> naga::ShaderStage {
        match self {
            Self::Vertex => naga::ShaderStage::Vertex,
            Self::Fragment => naga::ShaderStage::Fragment,
            Self::Compute => naga::ShaderStage::Compute,
        }
    }
}

pub(super) fn create_shader(device: &wgpu::Device, stage: ShaderStage, src: &str) -> Shader {
    let source = wgpu::ShaderSource::Glsl {
        shader: Cow::Borrowed(src),
        stage: stage.into(),
        defines: naga::FastHashMap::default(),
    };

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source,
    })
}
