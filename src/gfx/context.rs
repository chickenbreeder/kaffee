mod batch;
mod batch_ext;
mod buffer_ext;
mod pass;
mod pipeline_desc;
mod pipeline_ext;
mod text_ext;
mod texture_ext;

pub use batch_ext::BatchExt;
pub use buffer_ext::BufferExt;
pub use pipeline_desc::PipelineDescriptor;
pub use pipeline_ext::PipelineExt;
pub use text_ext::TextExt;
pub use texture_ext::TextureExt;

use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder};
use winit::window::Window;

use crate::{config::Config, error::ErrorKind, gfx::context::pipeline_ext::create_pipeline};

use self::{batch::Batch, pass::RenderPass};

use super::{
    buffer::{Buffer, MutableBuffer},
    camera::Camera,
    texture::{Texture, TextureRef},
    types::{Pipeline, Shader, ShaderStage, Vertex},
    Color,
};

const MAX_QUAD_COUNT: usize = 1000;
const DEFAULT_VERTEX_SHADER: &'static str = include_str!("../../res/shaders/default.vert.glsl");
const DEFAULT_FRAGMENT_SHADER: &'static str = include_str!("../../res/shaders/default.frag.glsl");

/// Enables basic operations like drawing or shader creation.
/// This type implements multiple extension traits such as [`TextureExt`] and [`BatchExt`] to keep the code cleaner and more readable.
pub struct GfxContext {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    texture_format: wgpu::TextureFormat,
    clear_color: Color,
    pipeline: Pipeline,
    batch: Batch<MAX_QUAD_COUNT>,
    staging_belt: wgpu::util::StagingBelt,
    render_passes: Vec<RenderPass>,
    default_texture: TextureRef,
    glyph_brush: GlyphBrush<()>,
    camera: Camera,
    camera_buffer: Buffer<Camera>,
    camera_bind_group: wgpu::BindGroup,
}

impl GfxContext {
    /// Creates a new [`GfxContext`]. When creating a new app, an instance of [`GfxContext`] will be created as well.
    pub async fn new(window: &Window, config: &Config) -> Result<Self, ErrorKind> {
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
            .expect("No suitable adapter was found");

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

        let texture_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
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

        let default_texture = Texture::from_bytes(
            &device,
            &queue,
            vec![255, 255, 255, 255],
            crate::prelude::FilterMode::Nearest,
        )?;

        let batch = Batch::new(&device);

        let default_font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("../../res/fonts/KenneyMini.ttf"))
                .expect("Failed to create default font");

        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let glyph_brush = GlyphBrushBuilder::using_font(default_font)
            .texture_filter_method(wgpu::FilterMode::Nearest)
            .build(&device, texture_format);

        let camera = Camera::new(config.width as f32, config.height as f32, 0., 0.);
        let camera_buffer = Buffer::from_data(&device, wgpu::BufferUsages::UNIFORM, &[camera]);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.handle().as_entire_binding(),
            }],
        });

        let pipeline = create_pipeline(
            &device,
            &PipelineDescriptor {
                vertex_shader,
                fragment_shader,
                texture_format,
            },
            &default_texture,
            &camera_bind_group_layout,
        );

        Ok(Self {
            instance,
            device,
            queue,
            surface,
            texture_format,
            clear_color: Color::BLACK,
            pipeline,
            batch,
            staging_belt,
            render_passes: vec![RenderPass {
                texture: default_texture.clone(),
            }],
            default_texture,
            glyph_brush,
            camera,
            camera_buffer,
            camera_bind_group,
        })
    }

    pub fn clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn create_shader(&self, stage: ShaderStage, src: &str) -> Shader {
        super::types::create_shader(&self.device, stage, src)
    }
}
