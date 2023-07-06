use crate::gfx::{types::Vertex, Color};

use super::GfxContext;

pub trait BatchExt {
    fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color);

    fn draw_quad(&mut self, x: f32, y: f32, w: f32, color: Color);

    /// Ends the current frame. This does not have to be called manually.
    fn end_frame(&mut self);
}

impl BatchExt for GfxContext {
    fn draw_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.vertices[self.vertices_off] = Vertex {
            position: [x + w, y + h, 0.0],
            color: color.into(),
        };
        self.vertices[self.vertices_off + 1] = Vertex {
            position: [x + w, y, 0.0],
            color: color.into(),
        };
        self.vertices[self.vertices_off + 2] = Vertex {
            position: [x, y, 0.0],
            color: color.into(),
        };
        self.vertices[self.vertices_off + 3] = Vertex {
            position: [x, y + h, 0.0],
            color: color.into(),
        };
        self.vertices_off += 4;
    }

    fn draw_quad(&mut self, x: f32, y: f32, w: f32, color: Color) {
        self.draw_rectangle(x, y, w, w, color)
    }

    fn end_frame(&mut self) {
        self.vertex_buffer.upload(&self.queue, &self.vertices);
        self.vertices_off = 0;

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
                        load: wgpu::LoadOp::Clear(self.clear_color.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, self.vertex_buffer.handle().slice(..));
            rpass.set_index_buffer(
                self.index_buffer.handle().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            rpass.draw_indexed(0..self.index_buffer.len() as u32, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}