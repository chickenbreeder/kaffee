use crate::gfx::{texture::Texture, types::Vertex, Color};

use super::GfxContext;

pub trait BatchExt {
    fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color);

    fn draw_quad(&mut self, x: f32, y: f32, w: f32, color: Color);

    fn draw_texture(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, texture: Texture);

    /// Ends the current frame. This does not have to be called manually.
    fn end_frame(&mut self);
}

impl BatchExt for GfxContext {
    fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let offset = self.batch.offset();

        self.batch[offset] = Vertex {
            position: [x + w, y + h, 0.0],
            color: color.into(),
            tex_coords: [1., 1.],
        };
        self.batch[offset + 1] = Vertex {
            position: [x + w, y, 0.0],
            color: color.into(),
            tex_coords: [1., 0.],
        };
        self.batch[offset + 2] = Vertex {
            position: [x, y, 0.0],
            color: color.into(),
            tex_coords: [0., 0.],
        };
        self.batch[offset + 3] = Vertex {
            position: [x, y + h, 0.0],
            color: color.into(),
            tex_coords: [0., 1.],
        };

        self.batch.set_offset(offset + 4);
    }

    fn draw_quad(&mut self, x: f32, y: f32, w: f32, color: Color) {
        self.draw_rectangle(x, y, w, w, color)
    }

    fn draw_texture(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, texture: Texture) {}

    fn end_frame(&mut self) {
        self.batch.flush(&self.queue);

        let output = self
            .surface
            .get_current_texture()
            .expect("Failed to retrieve current texture");
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
                        load: wgpu::LoadOp::Clear(self.clear_color.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, self.batch.vertex_buffer().handle().slice(..));
            rpass.set_bind_group(0, &self.camera_bind_group, &[]);
            rpass.set_bind_group(0, self.default_texture.bind_group(), &[]);
            rpass.set_index_buffer(
                self.batch.index_buffer().handle().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            rpass.draw_indexed(0..self.batch.index_buffer().len() as u32, 0, 0..1);
        }

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                1024,
                768,
            )
            .expect("Failed to draw text");

        self.staging_belt.finish();
        self.queue.submit(Some(encoder.finish()));
        output.present();
        self.staging_belt.recall();
    }
}
