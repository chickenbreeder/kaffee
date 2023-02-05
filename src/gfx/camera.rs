use glam::{Mat4, Quat, Vec3};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
]);

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Camera2D {
    pub view_projection: Mat4,
    pub model: Mat4,
}

impl Camera2D {
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> Self {
        let proj = Mat4::orthographic_rh_gl(0., width, height, 0., -1., 1.);
        let view = Mat4::from_scale_rotation_translation(
            Vec3::new(1., 1., 1.),
            Quat::IDENTITY,
            Vec3::new(x, y, 0.0),
        );
        let view_projection = OPENGL_TO_WGPU_MATRIX * proj * view;
        let model = Mat4::IDENTITY;

        Self {
            view_projection,
            model,
        }
    }
}
