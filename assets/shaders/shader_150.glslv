//132
//#version 140

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_light;

uniform mat4 projection;
uniform mat4 view;
uniform vec3 u_light;

uniform float f_seed;
uniform float f_persistance;
uniform float f_lacunarity;
uniform int i_octaves;

float Noise(vec4 p)
{
    return (0.5 * snoise(p) + 0.5);
}

float getNoiseValue(vec3 p, float scale) {
    float amplitude = 1;
    float frequency = 1;
    float value = 0;
    for (int i = 0; i < i_octaves; i++) {
        value += amplitude * Noise(vec4(frequency * p.x / scale, frequency * p.y / scale, frequency * p.z / scale, f_seed));
        amplitude *= f_persistance;
        frequency *= f_lacunarity;
    }
    return value;
}

void main() {
    v_normal = transpose(inverse(mat3(view))) * normal;
    v_light = transpose(inverse(mat3(view))) * u_light;
    vec3 pos3 = position;
    //pos3 *= Terrain(vec4(pos3, 1.0), 6, 36) * 10;
    pos3 = pos3 + (normalize(pos3) * getNoiseValue(pos3, 1.0)) * 0.1;
    vec4 pos = vec4(pos3, 1.0);
    pos = projection * view * pos;
    gl_Position = pos;
}