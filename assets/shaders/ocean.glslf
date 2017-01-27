#version 140

in vec3 v_normal;
in vec3 v_light;
in vec3 v_position;

uniform vec3 u_color;

out vec4 color;

const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    //color = vec4(0.0, 1.0, 0.0, 1.0);
    float brightness = max(dot(normalize(v_normal), normalize(v_light)), 0.0);
    vec3 dark_color = u_color * vec3(0.6, 0.6, 0.6);
    vec3 regular_color = u_color;

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(v_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    color = vec4(mix(dark_color, regular_color, brightness) + specular * specular_color, 1.0);
}