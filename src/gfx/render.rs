//! Types responsible for interacting with the GPU.

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use std::mem;

use super::{MAX_INDEXES_COUNT, MAX_QUAD_COUNT, MAX_VERTEX_COUNT};
use crate::gfx::Vertex;

use glam::vec2;
use wgpu::TextureFormat;
use winit::window::Window;

use crate::{
    error::ErrorKind,
    gfx::{batch_context::BatchContext, texture_atlas::TextureAtlas},
};

use super::{camera::Camera2D, color::Color, pipeline::BatchPipeline, texture::Texture2D};

/// The [`RenderContext`] allows interactions with the GPU.
pub struct RenderContext(InnerRenderContext);

impl Deref for RenderContext {
    type Target = InnerRenderContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RenderContext {
    pub async fn new(window: &Window) -> Self {
        let inner_context = InnerRenderContext::from_window(window).await;
        Self(inner_context)
    }
}

pub struct InnerRenderContext {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    supported_formats: Vec<TextureFormat>,
    batch_pipeline: BatchPipeline,
    camera: Camera2D,
    texture_atlas: TextureAtlas,
    default_batch_context: BatchContext,
}

impl InnerRenderContext {
    async fn from_window(window: &Window) -> Self {
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        let scale_factor = window.scale_factor() as f32;
        let logical_width = width as f32 / scale_factor;
        let logical_height = height as f32 / scale_factor;

        log::info!("Creating render context (w={width}, h={height}, sf={scale_factor})");

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let supported_formats = surface.get_supported_formats(&adapter);

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
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let vertex_shader = Self::create_shader(
            &device,
            naga::ShaderStage::Vertex,
            include_str!("../../res/shaders/default.vert.glsl"),
        );

        let fragment_shader = Self::create_shader(
            &device,
            naga::ShaderStage::Fragment,
            include_str!("../../res/shaders/default.frag.glsl"),
        );

        let camera = Camera2D::new(logical_width, logical_height, 0., 0.);
        let texture_atlas = TextureAtlas::from_path(
            "./res/textures/atlas.png",
            &device,
            &queue,
            2,
            2,
            vec2(16., 16.),
        )
        .unwrap();

        let batch_pipeline = BatchPipeline::new(
            &device,
            &supported_formats,
            vertex_shader,
            fragment_shader,
            &camera,
            &texture_atlas,
        );

        let default_batch_context =
            BatchContext::with_capacity(mem::size_of::<Vertex>() * MAX_VERTEX_COUNT as usize);

        Self {
            instance,
            device,
            queue,
            surface,
            supported_formats,
            batch_pipeline,
            camera,
            texture_atlas,
            default_batch_context,
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn supported_formats(&self) -> &[TextureFormat] {
        &self.supported_formats
    }

    fn create_shader(
        device: &wgpu::Device,
        stage: naga::ShaderStage,
        src: &str,
    ) -> wgpu::ShaderModule {
        let source = wgpu::ShaderSource::Glsl {
            shader: Cow::Borrowed(src),
            stage,
            defines: naga::FastHashMap::default(),
        };

        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source,
        })
    }

    pub fn draw_batch<B>(&mut self, batch_context: B)
    where
        B: Fn(&mut BatchContext),
    {
        batch_context(&mut self.default_batch_context);
        self.default_batch_context.reset();
    }

    pub fn end_frame(&mut self) {
        self.batch_pipeline
            .flush(&self.queue, &self.default_batch_context);

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.batch_pipeline.render_pipeline());
            rpass.set_bind_group(0, self.batch_pipeline.camera_bind_group(), &[]);
            rpass.set_bind_group(1, self.batch_pipeline.diffuse_bind_group(), &[]);
            rpass.set_vertex_buffer(0, self.batch_pipeline.vertex_buffer().handle().slice(..));
            rpass.set_index_buffer(
                self.batch_pipeline.index_buffer().handle().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            rpass.draw_indexed(0..self.batch_pipeline.index_buffer().len() as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn load_texture<P>(&self, path: P) -> Result<Texture2D, ErrorKind>
    where
        P: AsRef<std::path::Path>,
    {
        Texture2D::from_path(path, &self.device, &self.queue)
    }
}
