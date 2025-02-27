mod camera;
mod core;
mod ffis;
mod shaders;
mod state;

use core::objects::RenderObject;
use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;
use sokol::time;
use sokol::audio as aud;

use glam as glm;

use ffis::*;
use state::State;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1200;

fn main() {
    let state = State::new();
    let state = Box::new(state);
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
        window_title: c"compassion collective".as_ptr(),
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

fn callback_init(user_data: *mut c_void, state: &mut State) {
    time::setup();

    gfx::setup(&gfx::Desc {
        environment: glue::environment(),
        logger: gfx::Logger {
            func: Some(log::slog_func),
            user_data,
        },
        ..Default::default()
    });

    aud::setup(&aud::Desc {
        num_channels: 1,
        sample_rate: 44100,
        logger: aud::Logger {
            func: Some(log::slog_func),
            user_data,
        },
        ..Default::default()
    });

    #[rustfmt::skip]
    const TRI_VERTICES: [f32; 18] = [
        // vertices         colors
        -0.5, 1.5, 5.,     1., 0., 0.,
        0.5 , 1.5, 5.,     0., 1., 0.,
        0.  , 0.5 , 5.,     0., 0., 1.,
    ];

    #[rustfmt::skip]
    const TOMB_VERTICES: [f32; 144] = [
        // vertices         colors
        -0.5, -0.5,  0.1,   0.5, 0.5, 0.5,
         0.5, -0.5,  0.1,   0.5, 0.5, 0.5,
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
        -0.5, -0.5,  0.1,   0.5, 0.5, 0.5,

        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
         0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
         0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
         0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,

        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
        -0.5, -0.5,  0.1,   0.5, 0.5, 0.5,
        -0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
        -0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
        -0.5, -0.5, -0.1,   0.5, 0.5, 0.5,

         0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
         0.5, -0.5,  0.1,   0.5, 0.5, 0.5,
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
         0.5,  0.5,  0.1,   0.6, 0.6, 0.6,
         0.5,  0.5, -0.1,   0.6, 0.6, 0.6,
         0.5, -0.5, -0.1,   0.5, 0.5, 0.5,
    ];

    #[rustfmt::skip]
    const GROUND_VERTICES: [f32; 48] = [
        // vertcs                colors            tex uv
        -100.0, -0.5, -100.0,     0.3, 0.3, 0.3,     0., 0.,
        100.0 , -0.5, -100.0,     0.3, 0.3, 0.3,     0., 100.,
        100.0 , -0.5, 100.0 ,     0.3, 0.3, 0.3,     100., 100.,
        100.0 , -0.5, 100.0 ,     0.3, 0.3, 0.3,     100., 100.,
        -100.0, -0.5, 100.0 ,     0.3, 0.3, 0.3,     100., 0.,
        -100.0, -0.5, -100.0,     0.3, 0.3, 0.3,     0., 0.,
    ];

    let obj1 = RenderObject {
        vertex_buffer: gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&TOMB_VERTICES),
            ..Default::default()
        }),
        vertex_count: TOMB_VERTICES.len(),
        texture: None,
    };
    state.objects.push(obj1);

    let img = image::open("textures/ground.jpg").unwrap();
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let texture = gfx::make_image(&gfx::ImageDesc {
        width: width as i32,
        height: height as i32,
        pixel_format: gfx::PixelFormat::Rgba8,
        data: {
            let mut data = gfx::ImageData::new();
            data.subimage[0][0] = gfx::slice_as_range(&rgba);
            data
        },
        ..Default::default()
    });
    state.bindings.images[shaders::IMG_TEX] = texture;
    state.bindings.samplers[shaders::SMP_SMP] = gfx::make_sampler(&gfx::SamplerDesc {
        ..Default::default()
    });

    let obj1 = RenderObject {
        vertex_buffer: gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&GROUND_VERTICES),
            ..Default::default()
        }),
        vertex_count: GROUND_VERTICES.len(),
        texture: Some(state.bindings.samplers[0]),
    };
    state.objects.push(obj1);

    let obj1 = RenderObject {
        vertex_buffer: gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&TRI_VERTICES),
            ..Default::default()
        }),
        vertex_count: TRI_VERTICES.len(),
        texture: None,
    };
    state.objects.push(obj1);

    state.pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&shaders::untextured_shader_desc(gfx::query_backend())),
        primitive_type: gfx::PrimitiveType::Triangles,
        cull_mode: gfx::CullMode::None,
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[shaders::ATTR_UNTEXTURED_POSITION].format = gfx::VertexFormat::Float3;
            layout.attrs[shaders::ATTR_UNTEXTURED_A_COLOR].format = gfx::VertexFormat::Float3;
            layout
        },
        depth: gfx::DepthState {
            write_enabled: true,
            compare: gfx::CompareFunc::LessEqual,
            ..Default::default()
        },
        ..Default::default()
    });

    state.pipeline_textured = gfx::make_pipeline(&gfx::PipelineDesc {
        shader: gfx::make_shader(&shaders::textured_shader_desc(gfx::query_backend())),
        primitive_type: gfx::PrimitiveType::Triangles,
        cull_mode: gfx::CullMode::None,
        layout: {
            let mut layout = gfx::VertexLayoutState::new();
            layout.attrs[shaders::ATTR_TEXTURED_POSITION].format = gfx::VertexFormat::Float3;
            layout.attrs[shaders::ATTR_TEXTURED_A_COLOR].format = gfx::VertexFormat::Float3;
            layout.attrs[shaders::ATTR_TEXTURED_A_TEXCOORD].format = gfx::VertexFormat::Float2;
            layout
        },
        depth: gfx::DepthState {
            write_enabled: true,
            compare: gfx::CompareFunc::LessEqual,
            ..Default::default()
        },
        ..Default::default()
    });

    state.pass_action.colors[0] = gfx::ColorAttachmentAction {
        load_action: gfx::LoadAction::Clear,
        #[rustfmt::skip]
        clear_value: gfx::Color { r: 0.07, g: 0.01, b: 0.01, a: 1., },
        ..Default::default()
    };
}

fn callback_event(event: &sap::Event, state: &mut State) {
    state.inputs.get_inputs(event);
}

fn callback_frame(state: &mut State) {
    state.update_metrics();
    state.update_camera();
    state.display_fps();

    gfx::begin_pass(&gfx::Pass {
        action: state.pass_action,
        swapchain: glue::swapchain(),
        ..Default::default()
    });

    gfx::apply_viewport(15, 15, sap::width() - 30, sap::height() - 30, false);

    let projection = state.camera.projection_matrix();
    let view = state.camera.view_matrix();
    let model = glm::Mat4::IDENTITY;

    let vs_params = [model, view, projection];

    for obj in state.objects.iter() {
        if obj.texture.is_none() {
            gfx::apply_pipeline(state.pipeline);
        } else {
            gfx::apply_pipeline(state.pipeline_textured);
        }
        state.bindings.vertex_buffers[0] = obj.vertex_buffer;
        gfx::apply_bindings(&state.bindings);
        gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::value_as_range(&vs_params));
        gfx::draw(0, obj.vertex_count, 1);
    }

    gfx::end_pass();
    gfx::commit();

    let buffer_size = aud::buffer_frames();
    let mut buffer = vec![0.0; buffer_size as usize];

    (0..buffer_size as usize).for_each(|i| {
        if state.sample_pos >= state.sample_data.len() {
            state.sample_pos = 0;
        }
        buffer[i] = state.sample_data[state.sample_pos];
        state.sample_pos += 1;
    });

    if !buffer.is_empty() {
        aud::push(&buffer[0], buffer_size);
    }
}
