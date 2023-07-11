layout(location = 0) in vec4 v_Color;
layout(location = 1) in vec2 v_TexCoords;

layout(set = 1, binding = 0) uniform texture2D u_Texture;
layout(set = 1, binding = 1) uniform sampler u_Sampler;

out vec4 color;

void main()
{
    color = v_Color * texture(sampler2D(u_Texture, u_Sampler), v_TexCoords);
}
