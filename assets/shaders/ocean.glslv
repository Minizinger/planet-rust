#version 140

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_light;

uniform mat4 projection;
uniform mat4 view;
uniform vec3 u_light;
uniform float f_time;
uniform mat4 scale;

void main() {
    v_normal = transpose(inverse(mat3(view))) * normal;
    v_light = transpose(inverse(mat3(view))) * u_light;
    vec4 pos = vec4(position, 1.0);
    pos = pos * scale;
    //pos = pos + (normalize(pos) * sin(pos * 50 + f_time)) * 0.0025;
    pos = projection * view * pos;
    gl_Position = pos;
}