mod camera;
mod ffis;
mod shaders;

use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;

use glam as glm;

use camera::*;
use ffis::*;
use sokol::time;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1200;
#[rustfmt::skip]
const BACKGROUND: gfx::Color = gfx::Color { r: 0.1, g: 0.1, b: 0.1, a: 1., };

fn main() {
    let state = Box::new(State::new());
    let user_data = Box::into_raw(state) as *mut c_void;

    sap::run(&sap::Desc {
        user_data,
        init_userdata_cb: Some(ffi_cb_init),
        event_userdata_cb: Some(ffi_cb_event),
        frame_userdata_cb: Some(ffi_cb_frame),
        cleanup_userdata_cb: Some(ffi_cb_cleanup),
        width: WIDTH,
        height: HEIGHT,
        fullscreen: false,
        window_title: c"learn opengl with sokol and rust".as_ptr(),
        high_dpi: true,
        sample_count: 8,
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

struct RenderObject {
    mesh: Mesh,
    transform: glm::Mat4,
    pipeline: gfx::Pipeline,
}

struct Mesh {
    vertex_buffer: gfx::Buffer,
    index_buffer: Option<gfx::Buffer>,
    vertex_count: usize,
}

#[derive(Default)]
struct Metrics {
    frame_time: f32,
    last_frame_time: u64,
}

impl Metrics {
    pub fn update(&mut self) {
        let current_time = time::now();
        self.frame_time = time::sec(current_time - self.last_frame_time) as f32;
        self.last_frame_time = current_time;
    }
}

#[derive(Default)]
struct State {
    objects: Vec<RenderObject>,
    bindings: gfx::Bindings,
    pass_action: gfx::PassAction,

    camera: Camera,
    inputs: Inputs,

    metrics: Metrics,
}

impl State {
    fn new() -> State {
        State {
            camera: Camera::new(),
            ..Default::default()
        }
    }

    fn callback_init(&mut self, self_c_ptr: *mut c_void) {
        time::setup();
        gfx::setup(&gfx::Desc {
            environment: glue::environment(),
            logger: gfx::Logger {
                func: Some(log::slog_func),
                user_data: self_c_ptr,
            },
            ..Default::default()
        });

        #[rustfmt::skip]
        let vertices: [f32; 18] = [
            -0.5, -0.5, 0., 1., 0.7, 0.,
            0.5, -0.5, 0., 0., 1., 0.7,
            0., 0.5, 0., 0.7, 0., 1.,
        ];

        #[rustfmt::skip]
        let indices: [u16; 3] = [
            0, 1, 2
        ];

        let vertex_bindings = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&vertices),
            label: c"triangle buffer".as_ptr(),
            ..Default::default()
        });

        let index_bindings = gfx::make_buffer(&gfx::BufferDesc {
            _type: gfx::BufferType::Indexbuffer,
            data: gfx::slice_as_range(&indices),
            label: c"triangle index buffer".as_ptr(),
            ..Default::default()
        });

        let pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::simple_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            index_type: gfx::IndexType::Uint16,
            cull_mode: gfx::CullMode::None,
            depth: gfx::DepthState {
                compare: gfx::CompareFunc::LessEqual,
                ..Default::default()
            },
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shaders::ATTR_SIMPLE_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_SIMPLE_V_COLOR].format = gfx::VertexFormat::Float3;
                layout
            },
            label: c"simple draw pipeline".as_ptr(),
            ..Default::default()
        });

        let pass_action = gfx::ColorAttachmentAction {
            load_action: gfx::LoadAction::Clear,
            clear_value: BACKGROUND,
            ..Default::default()
        };

        let object = RenderObject {
            mesh: Mesh {
                vertex_buffer: vertex_bindings,
                index_buffer: Some(index_bindings),
                vertex_count: vertices.len() / 6,
            },
            transform: glm::Mat4::IDENTITY,
            pipeline,
        };

        self.objects.push(object);
        self.pass_action.colors[0] = pass_action;
    }

    fn callback_event(&mut self, event: &sap::Event) {
        self.inputs.get_inputs(event);
    }

    fn callback_frame(&mut self) {
        self.metrics.update();
        self.camera.update(&mut self.inputs, self.metrics.frame_time);

        gfx::begin_pass(&gfx::Pass {
            action: self.pass_action,
            swapchain: glue::swapchain(),
            ..Default::default()
        });
        gfx::apply_viewport(0, 0, sap::width(), sap::height(), false);

        let projection = self.camera.projection_matrix();
        let view = self.camera.view_matrix();

        for object in &self.objects {
            gfx::apply_pipeline(object.pipeline);

            let model = object.transform;

            let vs_params = [model, view, projection];

            self.bindings.vertex_buffers[0] = object.mesh.vertex_buffer;
            if let Some(index_buffer) = object.mesh.index_buffer {
                self.bindings.index_buffer = index_buffer;
            }
            gfx::apply_bindings(&self.bindings);
            gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::slice_as_range(&vs_params));

            gfx::draw(0, object.mesh.vertex_count, 1);
        }

        gfx::end_pass();
        gfx::commit();
    }
}
