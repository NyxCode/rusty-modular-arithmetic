#version 140

in vec2 position;
in vec3 color;

out vec3 vColor;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vColor = vec3((position.x + 1.0) / 2.0, (position.y + 1.0) / 2.0, 1.0);
}