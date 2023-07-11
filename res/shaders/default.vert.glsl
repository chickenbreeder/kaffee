layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec4 a_Color;
layout(location = 2) in vec2 a_TexCoords;

layout(location = 0) out vec4 v_Color;
layout(location = 1) out vec2 v_TexCoords;

layout(set = 0, binding = 0) uniform mat4 u_ViewProjection;

void main()
{
    v_Color = a_Color;
    v_TexCoords = a_TexCoords;
    gl_Position = u_ViewProjection * vec4(a_Pos, 1.0);
}
