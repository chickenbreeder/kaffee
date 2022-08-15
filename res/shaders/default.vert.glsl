layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

out vec4 v_Color;

void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = vec4(pos, 1.0);
}
