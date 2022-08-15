layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

out vec4 v_Color;

struct Camera2D
{
    mat4 ViewProjection;
    mat4 Model;
};

layout(binding = 0) uniform Camera2D u_Camera;

void main() {
    v_Color = vec4(color, 1.0);
    gl_Position =  u_Camera.ViewProjection * u_Camera.Model * vec4(pos, 1.0);
}
