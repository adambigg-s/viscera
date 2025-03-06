///////////////////////////////////////////////////////////////////////////////////////////////////////////
// simple shaders
@vs vert
in vec3 position;
in vec3 v_color;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec3 f_color;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);
    f_color = v_color;
}

@end

@fs frag
in vec3 f_color;

out vec4 color;

void main() {
    color = vec4(f_color, 1.);
}

@end

@program simple vert frag

///////////////////////////////////////////////////////////////////////////////////////////////////////////
// texture shaders
@vs tex_vert
in vec3 position;
in vec2 v_tex_pos;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec2 f_tex_pos;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);
    f_tex_pos = v_tex_pos;
}

@end

@fs tex_frag
in vec2 f_tex_pos;

layout (binding = 0) uniform texture2D tex;
layout (binding = 1) uniform sampler samp;

out vec4 color;

void main() {
    color = texture(sampler2D(tex, samp), f_tex_pos);
}

@end

@program texture tex_vert tex_frag

///////////////////////////////////////////////////////////////////////////////////////////////////////////
// solid-color shaders
@fs solid_frag
in vec2 f_tex_pos;

layout (binding = 1) uniform solid_params {
    vec4 solid_color;
};

out vec4 color;

void main() {
    color = solid_color;
}

@end

@program solid_color tex_vert solid_frag

///////////////////////////////////////////////////////////////////////////////////////////////////////////
// lighting shaders
@vs lighting_vert
in vec3 position;
in vec2 v_tex_pos;
in vec3 v_normal;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec2 f_tex_pos;
out vec3 f_world_pos;
out vec3 f_world_normal;

void main() {
    vec4 world_pos = model * vec4(position, 1.);
    gl_Position = projection * view * world_pos;
    f_tex_pos = v_tex_pos;
    f_world_pos = world_pos.xyz;
    f_world_normal = mat3(model) * v_normal;
}

@end

@fs lighting_frag
in vec2 f_tex_pos;
in vec3 f_world_pos;
in vec3 f_world_normal;

layout (binding = 0) uniform texture2D tex;
layout (binding = 1) uniform sampler samp;
layout (binding = 2) uniform lighting_params {
    vec3 light_pos;
};

out vec4 color;

void main() {
    vec3 lighting_dir = normalize(light_pos - f_world_pos);
    vec3 normal = normalize(f_world_normal);
    vec4 pre_color = texture(sampler2D(tex, samp), f_tex_pos);
    
    float ambient = 0.15;
    float diffuse = max(dot(normal, lighting_dir), 0.0);

    color = (ambient + diffuse) * pre_color;
}

@end

@program lighting lighting_vert lighting_frag
