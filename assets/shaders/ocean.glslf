#version 140

in vec3 v_normal;
in vec3 v_light;

uniform vec3 u_color;

out vec4 color;

void main() {
    //color = vec4(0.0, 1.0, 0.0, 1.0);
    float brightness = dot(normalize(v_normal), normalize(v_light));
    vec3 dark_color = u_color * vec3(0.6, 0.6, 0.6);
    vec3 regular_color = u_color;

    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}