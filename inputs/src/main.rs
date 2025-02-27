mod camera;
mod core;
mod ffis;
mod shaders;
mod state;

use core::objects::Entity;
use core::objects::RenderObject;
use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;
use sokol::time;

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

    #[rustfmt::skip]
    const TRI_VERTICES: [f32; 18] = [
        // vertices         colors
        -0.5, 1.5, 5.,     1., 1., 0.,
        0.5 , 1.5, 5.,     0., 1., 1.,
        0.  , 0.5 , 5.,     1., 0., 1.,
    ];

    #[rustfmt::skip]
    const CUBE_VERTS: [f32; 288] = [
        // Front face
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0, 0.0,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0, 0.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,  1.0, 0.0, 0.0,  0.0, 1.0,
        -0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  0.0, 0.0,
        // Back face
        -0.5, -0.5, -0.5,  0.0, 1.0, 0.0,  0.0, 0.0,
         0.5, -0.5, -0.5,  0.0, 1.0, 0.0,  1.0, 0.0,
         0.5,  0.5, -0.5,  0.0, 1.0, 0.0,  1.0, 1.0,
         0.5,  0.5, -0.5,  0.0, 1.0, 0.0,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0, 0.0,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0, 0.0,  0.0, 0.0,
        // Left face
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  0.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0, 1.0,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 0.0, 1.0,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0, 1.0,  0.0, 0.0,
        // Right face
         0.5, -0.5, -0.5,  1.0, 1.0, 0.0,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 1.0, 0.0,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0, 0.0,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0, 0.0,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0, 0.0,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0, 0.0,  0.0, 0.0,
        // Top face
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  0.0, 0.0,
         0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  1.0, 0.0,
         0.5,  0.5,  0.5,  0.0, 1.0, 1.0,  1.0, 1.0,
         0.5,  0.5,  0.5,  0.0, 1.0, 1.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0, 1.0,  0.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0, 1.0,  0.0, 0.0,
        // Bottom face
        -0.5, -0.5, -0.5,  1.0, 0.0, 1.0,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0, 1.0,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0, 1.0,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0, 1.0,  1.0, 1.0,
        -0.5, -0.5,  0.5,  1.0, 0.0, 1.0,  0.0, 1.0,
        -0.5, -0.5, -0.5,  1.0, 0.0, 1.0,  0.0, 0.0,
    ];
    #[rustfmt::skip]
    let voxel_positions = vec![
        glm::Vec3::new(0.0, 0.0, 0.7),
        glm::Vec3::new(1.0, 0.0, 0.7),
        glm::Vec3::new(2.0, 0.0, 0.7),
        glm::Vec3::new(3.0, 0.0, 1.7),
        glm::Vec3::new(4.0, 3.0, 10.7),
        glm::Vec3::new(0.0, 3.0, 10.7),
        glm::Vec3::new(0.0, 3.0, 10.0),
        glm::Vec3::new(0.0, 3.0, 10.0),
        glm::Vec3::new(0.0, 3.0, 10.0),
        glm::Vec3::new(0.0, 3.0, 10.0),
        glm::Vec3::new(1.0, 3.0, 10.0),
        glm::Vec3::new(2.0, 3.0, 10.0),
        glm::Vec3::new(3.0, 3.0, 10.0),
        glm::Vec3::new(4.0, 3.0, 0.0),
        glm::Vec3::new(3.0, 1.0, 0.0),
        glm::Vec3::new(3.0, 2.0, 0.0),
        glm::Vec3::new(3.0, 3.0, 0.0),
        glm::Vec3::new(3.7, 4.0, 0.0),
        glm::Vec3::new(3.7, 3.0, 1.0),
        glm::Vec3::new(4.7, 4.0, 0.0),
        glm::Vec3::new(3.7, 1.0, 0.0),
        glm::Vec3::new(9.7, 2.0, 0.0),
        glm::Vec3::new(9.7, 3.0, 0.0),
        glm::Vec3::new(9.0, 4.0, 0.0),
        glm::Vec3::new(13.0, 3.0, 1.0),
        glm::Vec3::new(13.0, 6.0, 0.0),
        glm::Vec3::new(13.0, 6.0, 0.0),
        glm::Vec3::new(3.0, 6.0, 0.0),
        glm::Vec3::new(3.0, 6.0, 0.0),
        glm::Vec3::new(3.0, 6.0, 0.0),
        glm::Vec3::new(3.0, 6.0, 1.0),
        glm::Vec3::new(4.0, 4.0, 1.0),
        glm::Vec3::new(3.0, 1.0, 1.0),
        glm::Vec3::new(3.0, 2.0, 1.0),
        glm::Vec3::new(5.0, 3.0, 1.0),
        glm::Vec3::new(5.0, 4.0, 1.0),
        glm::Vec3::new(5.0, 3.0, 1.0),
        glm::Vec3::new(5.0, 4.0, 0.0),
        glm::Vec3::new(5.0, 1.0, 0.0),
        glm::Vec3::new(5.0, 2.0, 0.0),
        glm::Vec3::new(3.0, 3.0, 0.0),
        glm::Vec3::new(3.0, 4.0, 0.0),
    ];

    let img = image::open("textures/ground.jpg").expect("failed to read texture");
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

    state.sampler = gfx::make_sampler(&gfx::SamplerDesc {
        min_filter: gfx::Filter::Linear,
        mag_filter: gfx::Filter::Linear,
        ..Default::default()
    });

    let cube_buffer = gfx::make_buffer(&gfx::BufferDesc {
        data: gfx::slice_as_range(&CUBE_VERTS),
        ..Default::default()
    });
    for pos in voxel_positions {
        let entity = Entity {
            render_object: RenderObject {
                vertex_buffer: cube_buffer,
                vertex_count: CUBE_VERTS.len() / 6,
            },
            position: pos,
            scale: glm::Vec3::new(1., 1., 1.),
            texture: Some(texture),
            ..Default::default()
        };
        state.entities.push(entity);
    }

    let triangle = gfx::make_buffer(&gfx::BufferDesc {
        data: gfx::slice_as_range(&TRI_VERTICES),
        ..Default::default()
    });
    let tri_entity = Entity {
        render_object: RenderObject {
            vertex_buffer: triangle,
            vertex_count: TRI_VERTICES.len() / 6,
        },
        scale: glm::Vec3::new(1., 1., 1.),
        position: glm::Vec3::new(0., 0., 0.),
        ..Default::default()
    };
    state.entities.push(tri_entity);

    state.pipeline_untextured = gfx::make_pipeline(&gfx::PipelineDesc {
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
            layout.attrs[shaders::ATTR_UNTEXTURED_POSITION].format = gfx::VertexFormat::Float3;
            layout.attrs[shaders::ATTR_UNTEXTURED_A_COLOR].format = gfx::VertexFormat::Float3;
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

    gfx::begin_pass(&gfx::Pass {
        action: state.pass_action,
        swapchain: glue::swapchain(),
        ..Default::default()
    });

    gfx::apply_viewport(15, 15, sap::width() - 30, sap::height() - 30, false);

    let projection = state.camera.projection_matrix();
    let view = state.camera.view_matrix();

    for &entity in &state.entities {
        let model = glm::Mat4::from_scale_rotation_translation(
            entity.scale,
            entity.rotation,
            entity.position,
        );
        let vs_params = [model, view, projection];

        if let Some(texture) = entity.texture {
            gfx::apply_pipeline(state.pipeline_textured);
            state.bindings.images[0] = texture;
            state.bindings.samplers[0] = state.sampler;
        } else {
            gfx::apply_pipeline(state.pipeline_untextured);
        }
        state.bindings.vertex_buffers[0] = entity.render_object.vertex_buffer;
        gfx::apply_bindings(&state.bindings);
        gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::value_as_range(&vs_params));
        gfx::draw(0, entity.render_object.vertex_count, 1);
    }

    gfx::end_pass();
    gfx::commit();
}
