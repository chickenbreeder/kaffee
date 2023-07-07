layout(location = 0) in vec4 v_Color;
layout(location = 1) in vec2 v_TexCoords;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

out vec4 color;

void main()
{
    color = v_Color * texture(sampler2D(t_diffuse, s_diffuse), v_TexCoords);
}
