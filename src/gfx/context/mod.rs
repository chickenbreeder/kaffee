mod batch_ext;
mod buffer_ext;
mod pipeline_ext;
mod texture_ext;
mod pipeline_desc;
mod pass;

pub use batch_ext::BatchExt;
pub use buffer_ext::BufferExt;
pub use pipeline_desc::PipelineDescriptor;
pub use pipeline_ext::PipelineExt;
pub use texture_ext::TextureExt;

use winit::window::Window;

use crate::{
    config::Config,
    gfx::{
        context::{
            buffer_ext::{create_buffer, create_buffer_mut},
            pipeline_ext::create_pipeline,
        },
        types::BufferUsages,
    },
};

use self::pass::RenderPass;

use super::{
    buffer::{Buffer, MutableBuffer},
    types::{Pipeline, Shader, ShaderStage, Vertex},
    Color, texture::{Texture, TextureRef},
};

const MAX_QUAD_COUNT: u64 = 1000;
const MAX_VERTEX_COUNT: u64 = MAX_QUAD_COUNT * 4;
const MAX_INDEX_COUNT: u64 = MAX_QUAD_COUNT * 6;

const DEFAULT_VERTEX_SHADER: &'static str = include_str!("../../../res/shaders/default.vert.glsl");
const DEFAULT_FRAGMENT_SHADER: &'static str =
    include_str!("../../../res/shaders/default.frag.glsl");

/// Enables basic operations like drawing or shader creation.
/// This type implements multiple extension traits such as [`TextureExt`] and [`BatchExt`] to keep the code cleaner and more readable.
pub struct GfxContext {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    clear_color: Color,
    pipeline: Pipeline,
    vertices: Vec<Vertex>,
    vertices_off: usize,
    index_buffer: Buffer<u16>,
    vertex_buffer: MutableBuffer<Vertex>,
    render_passes: Vec<RenderPass>,
    default_texture: TextureRef,
}

impl GfxContext {
    /// Creates a new [`GfxContext`]. When creating a new app, an instance of [`GfxContext`] will be created as well.
    pub async fn new(window: &Window, config: &Config) -> Self {
        let width = window.inner_size().width;
        let height = window.inner_size().height;

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });

        let surface = unsafe { instance.create_surface(window) }.expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to request an adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .expect("Failed to request a device");

        let surface_caps = surface.get_capabilities(&adapter);
        log::info!("{surface_caps:#?}");

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        use super::types::create_shader;

        let vertex_shader = create_shader(&device, ShaderStage::Vertex, DEFAULT_VERTEX_SHADER);

        let fragment_shader =
            create_shader(&device, ShaderStage::Fragment, DEFAULT_FRAGMENT_SHADER);

        let default_texture = Texture::from_bytes(&device, &queue,vec![255, 255, 255, 255], crate::prelude::FilterMode::Nearest).unwrap();

        let pipeline = create_pipeline(
            &device,
            &PipelineDescriptor {
                vertex_shader,
                fragment_shader,
                texture_format: surface_format,
            },
            &default_texture,
        );

        let mut vertices = Vec::with_capacity(MAX_VERTEX_COUNT as usize);
        unsafe {
            vertices.set_len(MAX_VERTEX_COUNT as usize);
        }

        let mut indices = Vec::with_capacity(MAX_INDEX_COUNT as usize);
        unsafe {
            indices.set_len(MAX_INDEX_COUNT as usize);
        }

        let mut offset = 0;

        for i in (0..MAX_INDEX_COUNT as usize).step_by(6) {
            indices[i] = 0 + offset;
            indices[i + 1] = 1 + offset;
            indices[i + 2] = 2 + offset;
            indices[i + 3] = 2 + offset;
            indices[i + 4] = 3 + offset;
            indices[i + 5] = 0 + offset;
            offset += 4;
        }

        let index_buffer = create_buffer(&device, BufferUsages::INDEX, &indices);
        let vertex_buffer = create_buffer_mut(&device, BufferUsages::VERTEX, MAX_VERTEX_COUNT);

        Self {
            instance,
            device,
            queue,
            surface,
            clear_color: Color::BLACK,
            pipeline,
            vertices,
            vertices_off: 0,
            index_buffer,
            vertex_buffer,
            render_passes: vec![
                RenderPass {
                    texture: default_texture.clone(),
                }
            ],
            default_texture,
        }
    }

    pub fn clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn create_shader(&self, stage: ShaderStage, src: &str) -> Shader {
        super::types::create_shader(&self.device, stage, src)
    }
}
