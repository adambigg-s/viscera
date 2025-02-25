package main

import "core:fmt"

import gl "vendor:OpenGL"
import glfw "vendor:glfw"

WIDTH :: 1600
HEIGHT :: 1200

// odinfmt: disable
VERTEX_SHADER ::
	`
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 vColor;

void main() {
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
   vColor = aColor;
}` +
	"\x00"


FRAG_SHADER ::
	`
#version 330 core
in vec3 vColor;

out vec4 FragColor;

void main()
{
	FragColor = vec4(vColor, 1.);
}` +
	"\x00"
// odinfmt: enable

main :: proc() {
	if !bool(glfw.Init()) {
		fmt.eprintln("failed to init glfw")
		return
	}
	defer glfw.Terminate()

	glfw.WindowHint(glfw.CONTEXT_VERSION_MAJOR, 3)
	glfw.WindowHint(glfw.CONTEXT_VERSION_MINOR, 3)
	glfw.WindowHint(glfw.OPENGL_PROFILE, glfw.OPENGL_CORE_PROFILE)
	glfw.WindowHint(glfw.DOUBLEBUFFER, 1)

	window := glfw.CreateWindow(WIDTH, HEIGHT, "OPENGL TRIANGLE", nil, nil)
	if window == nil {
		fmt.eprintfln("failed to create glfw window")
		return
	}
	defer glfw.DestroyWindow(window)

	glfw.MakeContextCurrent(window)
	glfw.SwapInterval(1)

	gl.load_up_to(3, 3, glfw.gl_set_proc_address)

	state := init_opengl()
	defer cleanup(&state)

	gl.ClearColor(0.2, 0.3, 0.3, 1.)

	for !glfw.WindowShouldClose(window) {
		if glfw.GetKey(window, glfw.KEY_ESCAPE) == glfw.PRESS {
			glfw.SetWindowShouldClose(window, true)
		}

		render(&state)
		glfw.SwapBuffers(window)
		glfw.PollEvents()
	}
}

State :: struct {
	vao:     u32,
	vbo:     u32,
	ebo:     u32,
	program: u32,
}

init_opengl :: proc() -> State {
	state: State
	
	// odinfmt: disable
	vertices := [?]f32 {
		// vertices 		colors
		-0.5, -0.5, 0., 	1., 0., 0.,
		0.5 , -0.5, 0., 	0., 1., 0.,
		0.  , 0.5 , 0., 	0., 0., 1.,
	};
	// odinfmt: enable

	indices := [?]u32{0, 1, 2}

	gl.GenVertexArrays(1, &state.vao)
	gl.BindVertexArray(state.vao)

	gl.GenBuffers(1, &state.vbo)
	gl.BindBuffer(gl.ARRAY_BUFFER, state.vbo)
	gl.BufferData(gl.ARRAY_BUFFER, len(vertices) * size_of(f32), &vertices[0], gl.STATIC_DRAW)

	gl.GenBuffers(1, &state.ebo)
	gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, state.ebo)
	gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, len(vertices) * size_of(u32), &indices, gl.STATIC_DRAW)

	gl.VertexAttribPointer(0, 3, gl.FLOAT, gl.FALSE, 6 * size_of(f32), uintptr(0))
	gl.EnableVertexAttribArray(0)
	gl.VertexAttribPointer(1, 3, gl.FLOAT, gl.FALSE, 6 * size_of(f32), uintptr(3 * size_of(f32)))
	gl.EnableVertexAttribArray(1)

	vert_shader := create_shader(gl.VERTEX_SHADER, VERTEX_SHADER)
	frag_shader := create_shader(gl.FRAGMENT_SHADER, FRAG_SHADER)

	state.program = gl.CreateProgram()
	gl.AttachShader(state.program, vert_shader)
	gl.AttachShader(state.program, frag_shader)
	gl.LinkProgram(state.program)

	success: i32
	gl.GetProgramiv(state.program, gl.LINK_STATUS, &success)
	if cast(bool)success == gl.FALSE {
		info: [512]u8
		gl.GetProgramInfoLog(state.program, 512, nil, &info[0])
		fmt.eprintln("program linking failure:", string(info[:]))
	}

	gl.DeleteShader(vert_shader)
	gl.DeleteShader(frag_shader)

	return state
}

create_shader :: proc(type: u32, source: string) -> u32 {
	shader := gl.CreateShader(type)
	src := cstring(raw_data(source))
	gl.ShaderSource(shader, 1, &src, nil)
	gl.CompileShader(shader)

	success: i32
	gl.GetShaderiv(shader, gl.COMPILE_STATUS, &success)
	if cast(bool)success == gl.FALSE {
		info: [512]u8
		gl.GetShaderInfoLog(shader, 512, nil, &info[0])
		fmt.eprintln("shader comp failed:", string(info[:]))
	}

	return shader
}

render :: proc(state: ^State) {
	gl.Clear(gl.COLOR_BUFFER_BIT)

	gl.UseProgram(state.program)
	gl.BindVertexArray(state.vao)
	gl.DrawElements(gl.TRIANGLES, 3, gl.UNSIGNED_INT, nil)
}

cleanup :: proc(state: ^State) {
	gl.DeleteProgram(state.program)
	gl.DeleteBuffers(1, &state.vbo)
	gl.DeleteBuffers(1, &state.ebo)
	gl.DeleteVertexArrays(1, &state.vao)
}
