@vs simple_verts
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 a_color;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec3 color;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);;
    color = a_color;
}
@end

@fs simple_frags
in vec3 color;

out vec4 frag_color;

void main() {
    frag_color = vec4(color, 1.);
}
@end

@program most_basic simple_verts simple_frags
