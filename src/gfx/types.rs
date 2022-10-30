use std::{ops::Deref, sync::Arc};

pub struct Shader(Arc<wgpu::ShaderModule>);

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum ShaderType {
    Vertex = naga::ShaderStage::Vertex as isize,
    Fragment = naga::ShaderStage::Fragment as isize,
    Compute = naga::ShaderStage::Compute as isize,
}

impl Shader {
    pub(crate) fn new(inner: wgpu::ShaderModule) -> Self {
        Self(Arc::new(inner))
    }
}

impl Into<naga::ShaderStage> for ShaderType {
    fn into(self) -> naga::ShaderStage {
        match self {
            Self::Vertex => naga::ShaderStage::Vertex,
            Self::Fragment => naga::ShaderStage::Fragment,
            Self::Compute => naga::ShaderStage::Compute,
        }
    }
}

impl Deref for Shader {
    type Target = wgpu::ShaderModule;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
