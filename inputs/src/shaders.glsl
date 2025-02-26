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

out float fog_factor;
out vec2 texcoord;

void main() {
    vec4 world_pos = model * vec4(position, 1.);
    vec4 view_pos = view * world_pos;
    gl_Position = projection * view_pos;
    fog_factor = -view_pos.z;
    texcoord = a_texcoord;
}
@end

@fs textured_frags
in float fog_factor;
in vec2 texcoord;

layout (binding = 0) uniform texture2D tex;
layout (binding = 0) uniform sampler smp;

out vec4 frag_color;

void main() {
    float fog_start = 0.1;
    float fog_end = 5.;
    vec3 fog_color = vec3(.05, .05, .05);
    float fog_amount = clamp((fog_factor - fog_start) / (fog_end - fog_start), 0.0, 1.0);

    vec4 tex_color = texture(sampler2D(tex, smp), texcoord);

    vec3 final_color = mix(tex_color.rgb, fog_color, fog_amount);
    frag_color = vec4(final_color, 1.);
}
@end

@program textured textured_verts textured_frags

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
out float fog_factor;

void main() {
    vec4 world_pos = model * vec4(position, 1.);
    vec4 view_pos = view * world_pos;
    gl_Position = projection * view_pos;
    color = a_color;
    fog_factor = -view_pos.z;
}
@end

@fs untextured_frags
in vec3 color;
in float fog_factor;

out vec4 frag_color;

void main() {
    float fog_start = 0.1;
    float fog_end = 10.;
    vec3 fog_color = vec3(.1, .07, .07);
    float fog_amount = clamp((fog_factor - fog_start) / (fog_end - fog_start), 0.0, 1.0);

    vec3 final_color = mix(color, fog_color, fog_amount);
    frag_color = vec4(final_color, 1.);
}
@end

@program untextured untextured_verts untextured_frags
