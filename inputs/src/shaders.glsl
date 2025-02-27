// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // 
// textured shader pipeline
@vs textured_verts
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 a_color;
layout (location = 2) in vec2 a_texcoord;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec2 texcoord;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);
    texcoord = a_texcoord;
}
@end

@fs textured_frags
in vec2 texcoord;

layout (binding = 0) uniform texture2D tex;
layout (binding = 0) uniform sampler smp;
layout (binding = 1) uniform fs_params {
    float time;
};

out vec4 frag_color;

float random(vec2 uv) {
    return fract(sin(dot(uv.xy, vec2(123333.33, 100000.88))) * 5.);
}

void main() {
    vec4 base_color = texture(sampler2D(tex, smp), texcoord);
    vec2 scaled_coords = floor(texcoord * 200.) / 200.;
    float time_const = floor(time * 5.) / 5.;
    float noise = random(scaled_coords + vec2(sin(time_const), cos(time_const)));
    float dither = step(0.5, noise);

    vec3 color_shift = vec3(dither * 0.7);

    frag_color = vec4(base_color.rgb * (1. - color_shift), base_color.a);
}
@end

@program textured textured_verts textured_frags

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // 
// untextured shader pipeline
@vs untextured_verts
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 a_color;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec3 color;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);
    color = a_color;
}
@end

@fs untextured_frags
in vec3 color;

out vec4 frag_color;

void main() {
    frag_color = vec4(color, 1.);
}
@end

@program untextured untextured_verts untextured_frags
