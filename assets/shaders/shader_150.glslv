//132
//#version 140

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_light;

uniform mat4 projection;
uniform mat4 view;
uniform vec3 u_light;

float Noise(vec4 p)
{
    return (0.5 * snoise(p) + 0.5);
}

float Terrain(vec4 p, int steps, float _scale) //Scale = 36
{
    vec4 displace = vec4(0);
    for(int i = 0; i < steps; i++)
    {
        displace = vec4(
            Noise(p * _scale + displace),
            Noise(p.yzxw * _scale + displace),
            Noise(p.zxyw * _scale + displace),
            1.0);
            _scale *= 0.5f;
    }
    float e = Noise(p * _scale + displace);

    e = pow(e, 2);
    //float continent = Noise(p * .2f) > 0.5 ? .1f : (float)Math.Pow((Noise(p * .2f) * 2), 3) * .1f 0;
    //float continent = (float)Math.Pow(Noise(p * .2f), 2) * .3f;
    float continent = Noise(p * .2f) * .2f;
    int mask = continent > .1f ? 1 : 0;

    //float e = Noise(p) + .5f * Noise(p * 2) + .25f * Noise(p * 4) * mask;

    return continent + e * mask * .05f;
}

void main() {
    v_normal = transpose(inverse(mat3(view))) * normal;
    v_light = transpose(inverse(mat3(view))) * u_light;
    vec3 pos3 = position;
    pos3 *= Terrain(vec4(pos3, 1.0), 6, 36) * 10;
    vec4 pos = vec4(pos3, 1.0);
    pos = projection * view * pos;
    gl_Position = pos;
}