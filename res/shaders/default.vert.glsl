layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec4 a_Color;

layout(location = 0) out vec4 v_Color;

layout(binding = 0) uniform mat4 ViewProjection = mat4(1.0);

void main()
{
    v_Color = a_Color;
    gl_Position =  vec4(a_Pos, 1.0);
}
