use std::path::Path;

use image::GenericImageView;

use crate::{error::ErrorKind, fs};

pub struct Texture2D {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    width: u32,
    height: u32,
}

impl Texture2D {
    pub(crate) fn from_path<P>(
        path: P,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, ErrorKind>
    where
        P: AsRef<Path>,
    {
        let bytes = fs::load_file(path)?;
        Texture2D::from_bytes(device, queue, &bytes)
    }

    pub(crate) fn from_bytes(device: &wgpu::Device, queue: &wgpu::Queue, bytes: &[u8]) -> Result<Self, ErrorKind> {
        let img = image::load_from_memory(bytes)?;
        let diffuse_rgba = img.to_rgba8();
        let dim = img.dimensions();
        let width = dim.0;
        let height = dim.1;

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dim.0),
                rows_per_image: std::num::NonZeroU32::new(dim.1),
            },
            texture_size,
        );

        let texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture: diffuse_texture,
            view: texture_view,
            sampler: diffuse_sampler,
            width,
            height,
        })
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub(crate) fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub(crate) fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

}
