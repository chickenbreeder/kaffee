layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Color;
layout(location = 2) in vec2 a_TexCoords;

layout(location = 0) out vec4 v_Color;
layout(location = 1) out vec2 v_TexCoords;

struct Camera2D
{
    mat4 ViewProjection;
    mat4 Model;
};

layout(binding = 0) uniform Camera2D u_Camera;

void main() {
    v_Color = vec4(a_Color, 1.0);
    v_TexCoords = a_TexCoords;
    gl_Position =  u_Camera.ViewProjection * u_Camera.Model * vec4(a_Pos, 1.0);
}
