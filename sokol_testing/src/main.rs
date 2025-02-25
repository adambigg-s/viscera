mod shader;

use std::ffi::c_void;

use sok::app as sap;
use sok::gfx;
use sok::glue;
use sok::log;
use sokol as sok;

const HEIGHT: i32 = 1200;
const WIDTH: i32 = 1600;

fn main() {
    let state = Box::new(State {
        pass_actions: gfx::PassAction::new(),
        pipeline: gfx::Pipeline::new(),
        bindings: gfx::Bindings::new(),
    });
    let user_data = Box::into_raw(state) as *mut c_void;

    sap::run(&sap::Desc {
        user_data,
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        event_userdata_cb: Some(event),
        window_title: c"".as_ptr(),
        ios_keyboard_resizes_canvas: false,
        logger: sap::Logger {
            func: Some(log::slog_func),
            ..Default::default()
        },
        icon: sap::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        high_dpi: true,
        width: WIDTH,
        height: HEIGHT,
        ..Default::default()
    });
}

pub extern "C" fn init(user_data: *mut c_void) {
    unsafe {
        let state = &mut *(user_data as *mut State);

        sok::time::setup();

        gfx::setup(&gfx::Desc {
            environment: glue::environment(),
            logger: gfx::Logger {
                func: Some(log::slog_func),
                ..Default::default()
            },
            ..Default::default()
        });

        #[rustfmt::skip]
        const VERTICES: [f32; 18] = [
            // vertices              colors
            -0.5, -0.5, 0.,         1., 0., 0.,
             0.5, -0.5, 0.,         0., 1., 0.,
             0.,   0.5, 0.,         0., 0., 1.,
        ];
        #[rustfmt::skip]
        const INDICES: [u16; 3] = [
            0, 1, 2
        ];

        state.bindings.vertex_buffers[0] = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&VERTICES),
            ..Default::default()
        });
        state.bindings.index_buffer = gfx::make_buffer(&gfx::BufferDesc {
            _type: gfx::BufferType::Indexbuffer,
            data: gfx::slice_as_range(&INDICES),
            ..Default::default()
        });

        state.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shader::simple_shader_desc(gfx::query_backend())),
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shader::ATTR_SIMPLE_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shader::ATTR_SIMPLE_ACOLOR].format = gfx::VertexFormat::Float3;
                layout
            },
            index_type: gfx::IndexType::Uint16,
            primitive_type: gfx::PrimitiveType::Triangles,
            ..Default::default()
        });

        state.pass_actions.colors[0] = gfx::ColorAttachmentAction {
            load_action: gfx::LoadAction::Clear,
            clear_value: gfx::Color {
                r: 0.2,
                g: 0.3,
                b: 0.3,
                a: 1.,
            },
            ..Default::default()
        };
    }
}

pub extern "C" fn frame(user_data: *mut c_void) {
    unsafe {
        let state = &mut *(user_data as *mut State);

        gfx::begin_pass(&gfx::Pass {
            action: state.pass_actions,
            swapchain: glue::swapchain(),
            ..Default::default()
        });
        gfx::apply_viewport(0, 0, WIDTH, HEIGHT, false);
        gfx::apply_pipeline(state.pipeline);
        gfx::apply_bindings(&state.bindings);
        gfx::draw(0, 3, 1);
        gfx::end_pass();
        gfx::commit();
    }
}

pub extern "C" fn cleanup(user_data: *mut c_void) {
    gfx::shutdown();

    user_data.raw_drop();
}

#[rustfmt::skip]
#[allow(clippy::single_match)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn event(event: *const sap::Event, _user_data: *mut c_void) {
    unsafe {
        let event = &*event;
        match event._type {
            sap::EventType::KeyDown => match event.key_code {
                sap::Keycode::Escape => {
                    sap::request_quit();
                }
                sap::Keycode::Q => {
                    let gl_backend = gfx::query_backend();
                    println!("using backend: {:?}", gl_backend);
                }
                _ => {}
            }
            _ => {}
        }
    }
}

pub struct State {
    pub pipeline: gfx::Pipeline,
    pub bindings: gfx::Bindings,
    pub pass_actions: gfx::PassAction,
}

pub trait Droppable {
    fn raw_drop(self);
}

impl<T> Droppable for *mut T {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[allow(unused_must_use)]
    fn raw_drop(self) {
        unsafe {
            Box::from_raw(self);
        }
    }
}
