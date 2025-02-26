mod camera;
mod shaders;
mod state;

use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;
use sokol::time;

use glam as glm;

use state::RenderObject;
use state::State;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1200;

fn main() {
    let state = State::new();
    let state = Box::new(state);
    let user_data = Box::into_raw(state) as *mut c_void;

    sap::run(&sap::Desc {
        user_data,
        init_userdata_cb: Some(init),
        event_userdata_cb: Some(event),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        width: WIDTH,
        height: HEIGHT,
        window_title: c"horror game".as_ptr(),
        logger: sap::Logger {
            func: Some(log::slog_func),
            user_data,
        },
        icon: sap::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

extern "C" fn init(user_data: *mut c_void) {
    let state: &mut State;
    unsafe {
        state = &mut *(user_data as *mut State);
    }

    time::setup();
    gfx::setup(&gfx::Desc {
        environment: glue::environment(),
        logger: gfx::Logger {
            func: Some(log::slog_func),
            user_data,
        },
        ..Default::default()
    });

    #[rustfmt::skip]
    const TRI_VERTICES: [f32; 18] = [
        // vertices         colors
        -0.5, 3., 0.,     1., 0.,     0.,
         0.5, 3., 0.,         0., 1., 0.,
         0.,   4., 0.,         0.,     0., 1.,
    ];

    #[rustfmt::skip]
    const TOMB_VERTICES: [f32; 144] = [
        // Front face (position: x, y, z; color: r, g, b)
        -0.5, -0.5,  0.1,   0.5, 0.5, 0.5,  // Bottom-left
         0.5, -0.5,  0.1,   0.5, 0.5, 0.5,  // Bottom-right
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,  // Top-right
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,  // Top-right
        -0.5,  0.5,  0.1,   0.6, 0.6, 0.6,  // Top-left
        -0.5, -0.5,  0.1,   0.5, 0.5, 0.5,  // Bottom-left

        // Back face
        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
         0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
         0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
         0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,

        // Left face
        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
        -0.5, -0.5,  0.1,   0.5, 0.5, 0.5,
        -0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,

        // Right face
         0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
         0.5, -0.5,  0.1,   0.5, 0.5, 0.5,
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
         0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
         0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
    ];

    #[rustfmt::skip]
    const GROUND_VERTICES: [f32; 36] = [
        -10.0, -0.5, -10.0,  0.3, 0.3, 0.3,  // Bottom-left
        10.0, -0.5, -10.0,  0.3, 0.3, 0.3,  // Bottom-right
        10.0, -0.5,  10.0,  0.3, 0.3, 0.3,  // Top-right
        10.0, -0.5,  10.0,  0.3, 0.3, 0.3,  // Top-right
        -10.0, -0.5,  10.0,  0.3, 0.3, 0.3,  // Top-left
        -10.0, -0.5, -10.0,  0.3, 0.3, 0.3,  // Bottom-left
    ];

    let obj1 = RenderObject {
        vertex_buffer: gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&TOMB_VERTICES),
            ..Default::default()
        }),
        vertex_count: TOMB_VERTICES.len(),
    };
    state.objects.push(obj1);

    let obj1 = RenderObject {
        vertex_buffer: gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&GROUND_VERTICES),
            ..Default::default()
        }),
        vertex_count: GROUND_VERTICES.len(),
    };
    state.objects.push(obj1);

    let obj1 = RenderObject {
        vertex_buffer: gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&TRI_VERTICES),
            ..Default::default()
        }),
        vertex_count: TRI_VERTICES.len(),
    };
    state.objects.push(obj1);

    state.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&shaders::most_basic_shader_desc(gfx::query_backend())),
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[shaders::ATTR_MOST_BASIC_POSITION].format = gfx::VertexFormat::Float3;
            layout.attrs[shaders::ATTR_MOST_BASIC_A_COLOR].format = gfx::VertexFormat::Float3;
            layout
        },
        depth: gfx::DepthState {
            write_enabled: true,
            compare: gfx::CompareFunc::LessEqual,
            ..Default::default()
        },
        primitive_type: gfx::PrimitiveType::Triangles,
        ..Default::default()
    });

    state.pass_action.colors[0] = gfx::ColorAttachmentAction {
        load_action: gfx::LoadAction::Clear,
        #[rustfmt::skip]
        clear_value: gfx::Color { r: 0.2, g: 0.2, b: 0.2, a: 1., },
        ..Default::default()
    };
}

extern "C" fn cleanup(user_data: *mut c_void) {
    gfx::shutdown();
    #[allow(unused_must_use)]
    #[allow(clippy::from_raw_with_void_ptr)]
    unsafe {
        Box::from_raw(user_data);
    }
}

extern "C" fn event(raw_event: *const sap::Event, user_data: *mut c_void) {
    let event: &sap::Event;
    let state: &mut State;
    unsafe {
        event = &*raw_event;
        state = &mut *(user_data as *mut State);
    }
    state.inputs.get_inputs(event);
}

extern "C" fn frame(user_data: *mut c_void) {
    let state: &mut State;
    unsafe {
        state = &mut *(user_data as *mut State);
    }
    state.update_camera();

    gfx::begin_pass(&gfx::Pass {
        action: state.pass_action,
        swapchain: glue::swapchain(),
        ..Default::default()
    });

    gfx::apply_viewport(15, 15, sap::width() - 30, sap::height() - 30, false);
    gfx::apply_pipeline(state.pipeline);

    let projection = state.camera.projection_matrix();
    let view = state.camera.view_matrix();
    let model = glm::Mat4::IDENTITY;

    let vs_params = [model, view, projection];

    gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::value_as_range(&vs_params));

    for obj in state.objects.iter() {
        state.bindings.vertex_buffers[0] = obj.vertex_buffer;
        gfx::apply_bindings(&state.bindings);
        gfx::draw(0, obj.vertex_count, 1);
    }

    gfx::end_pass();
    gfx::commit();
}
