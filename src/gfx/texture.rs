use std::{path::Path, sync::Arc};

use image::RgbaImage;
use wgpu::{BindGroup, BindGroupLayout};

use crate::{error::ErrorKind, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Linear,
    Nearest,
}

impl Into<wgpu::FilterMode> for FilterMode {
    fn into(self) -> wgpu::FilterMode {
        match self {
            FilterMode::Linear => wgpu::FilterMode::Linear,
            FilterMode::Nearest => wgpu::FilterMode::Nearest,
        }
    }
}

pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    width: u32,
    height: u32,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
}

#[derive(Clone)]
pub struct TextureRef(Arc<Texture>);

impl std::ops::Deref for TextureRef {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Texture {
    pub(crate) fn from_path<P>(
        path: P,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        filter_mode: FilterMode,
    ) -> Result<TextureRef, ErrorKind>
    where
        P: AsRef<Path>,
    {
        let bytes = fs::load_file(path)?;
        let dyn_img = image::load_from_memory(&bytes).expect("Failed to create image");
        let rgba_image: RgbaImage = dyn_img.to_rgba8();

        Self::new(device, queue, &rgba_image, filter_mode)
    }

    pub(crate) fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: Vec<u8>,
        filter_mode: FilterMode,
    ) -> Result<TextureRef, ErrorKind> {
        //let data: Vec<u8> = vec![255, 255, 255, 255];
        let rgba_image: RgbaImage = RgbaImage::from_raw(1, 1, bytes).expect("Failed to cr");

        Self::new(device, queue, &rgba_image, filter_mode)
    }

    fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &RgbaImage,
        filter_mode: FilterMode,
    ) -> Result<TextureRef, ErrorKind> {
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
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dim.0),
                rows_per_image: Some(dim.1),
            },
            texture_size,
        );

        let texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let filter_mode = filter_mode.into();

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: filter_mode,
            ..Default::default()
        });

        let (bind_group_layout, bind_group) =
            Self::create_bind_group(device, &texture_view, &sampler);

        Ok(TextureRef(Arc::new(Self {
            texture: diffuse_texture,
            view: texture_view,
            sampler,
            width,
            height,
            bind_group_layout,
            bind_group,
        })))
    }

    pub(super) fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub(super) fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn create_bind_group(
        device: &wgpu::Device,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });

        (bind_group_layout, bind_group)
    }
}
