@vs vs
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 acolor;
out vec3 our_color;

void main() {
    gl_Position = vec4(position, 1.);
    our_color = acolor;
}
@end

@fs fs
in vec3 our_color;
out vec4 FragColor;

void main() {
    FragColor = vec4(our_color, 1.);
}
@end

@program simple vs fs

