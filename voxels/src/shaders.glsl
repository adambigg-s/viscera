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
out vec3 f_normal;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);
    f_tex_pos = v_tex_pos;
    f_normal = v_normal;
}

@end

@fs lighting_frag
in vec2 f_tex_pos;
in vec3 f_normal;
layout (binding = 0) uniform texture2D tex;
layout (binding = 1) uniform sampler samp;

out vec4 color;

void main() {
    vec3 lighting_dir = normalize(vec3(1., 1., 5.));
    vec3 normal = normalize(f_normal);
    vec4 pre_color = texture(sampler2D(tex, samp), f_tex_pos);
    float ambient = 0.15;
    float diffuse = max(dot(f_normal, lighting_dir), 0.0);

    color = (ambient + diffuse) * pre_color;
}

@end

@program lighting lighting_vert lighting_frag
