mod camera;
mod ffis;
mod shaders;
mod utils;

use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;
use sokol::time;

use glam as glm;

use camera::*;
use ffis::*;
use utils::Metrics;

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

enum Material {
    Simple,
    Texture { texture: gfx::Image },
    SolidColor { color: glm::Vec4 },
}

impl Default for Material {
    fn default() -> Material {
        Material::SolidColor {
            color: glm::Vec4::new(0., 1., 1., 1.),
        }
    }
}

#[derive(Default)]
struct Mesh {
    vertex_buffer: gfx::Buffer,
    index_buffer: Option<gfx::Buffer>,
    element_count: usize,
}

#[derive(Default)]
struct RenderObject {
    mesh: Mesh,
    transform: glm::Mat4,
    pipeline: gfx::Pipeline,
    material: Material,
}

#[derive(Default)]
struct State {
    objects: Vec<RenderObject>,
    bindings: gfx::Bindings,
    pass_action: gfx::PassAction,
    sampler: gfx::Sampler,

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
        let tri_vertices: [f32; 18] = [
            -0.5, -0.5, 0., 1., 0.7, 0.,
            0.5, -0.5, 0., 0., 1., 0.7,
            0., 0.5, 0., 0.7, 0., 1.,
        ];
        #[rustfmt::skip]
        let tri_indices: [u16; 3] = [
            0, 1, 2
        ];
        let tri_vert_bindings = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&tri_vertices),
            label: c"triangle buffer".as_ptr(),
            ..Default::default()
        });
        let tri_index_bindings = gfx::make_buffer(&gfx::BufferDesc {
            _type: gfx::BufferType::Indexbuffer,
            data: gfx::slice_as_range(&tri_indices),
            label: c"triangle index buffer".as_ptr(),
            ..Default::default()
        });
        let tri_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::simple_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            index_type: gfx::IndexType::Uint16,
            cull_mode: gfx::CullMode::None,
            depth: gfx::DepthState {
                compare: gfx::CompareFunc::Less,
                write_enabled: true,
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
        for idx in 0..100 {
            let object = RenderObject {
                mesh: Mesh {
                    vertex_buffer: tri_vert_bindings,
                    index_buffer: Some(tri_index_bindings),
                    element_count: tri_indices.len(),
                },
                transform: glm::Mat4::from_translation(glm::Vec3::new(0., 0., idx as f32)),
                pipeline: tri_pipeline,
                material: Material::Simple,
            };
            self.objects.push(object);
        }

        #[rustfmt::skip]
        let cube_verts: [f32; 180] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];
        let cube_vert_bindings = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&cube_verts),
            label: c"square texture verts".as_ptr(),
            ..Default::default()
        });
        let cube_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::texture_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            cull_mode: gfx::CullMode::None,
            depth: gfx::DepthState {
                compare: gfx::CompareFunc::LessEqual,
                write_enabled: true,
                ..Default::default()
            },
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shaders::ATTR_TEXTURE_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_TEXTURE_V_TEX_POS].format = gfx::VertexFormat::Float2;
                layout
            },
            label: c"texture draw pipeline".as_ptr(),
            ..Default::default()
        });
        let texture = load_texture("textures/num_grid.png");
        for dy in 0..20 {
            for dx in 0..20 {
                let object = RenderObject {
                    mesh: Mesh {
                        vertex_buffer: cube_vert_bindings,
                        index_buffer: None,
                        element_count: cube_verts.len() / 5,
                    },
                    transform: glm::Mat4::from_translation(glm::Vec3::new(
                        dx as f32, -1., dy as f32,
                    )),
                    pipeline: cube_pipeline,
                    material: Material::Texture { texture },
                };
                self.objects.push(object);
            }
        }

        #[rustfmt::skip]
        let lighting_cube_verts: [f32; 288] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,  1., 0., 0.,
             0.5, -0.5, -0.5,  1.0, 0.0,  1., 0., 0.,
             0.5,  0.5, -0.5,  1.0, 1.0,  1., 0., 0.,
             0.5,  0.5, -0.5,  1.0, 1.0,  1., 0., 0.,
            -0.5,  0.5, -0.5,  0.0, 1.0,  1., 0., 0.,
            -0.5, -0.5, -0.5,  0.0, 0.0,  1., 0., 0.,

            -0.5, -0.5,  0.5,  0.0, 0.0,  0., 1., 0.,
             0.5, -0.5,  0.5,  1.0, 0.0,  0., 1., 0.,
             0.5,  0.5,  0.5,  1.0, 1.0,  0., 1., 0.,
             0.5,  0.5,  0.5,  1.0, 1.0,  0., 1., 0.,
            -0.5,  0.5,  0.5,  0.0, 1.0,  0., 1., 0.,
            -0.5, -0.5,  0.5,  0.0, 0.0,  0., 1., 0.,

            -0.5,  0.5,  0.5,  1.0, 0.0,  0., -1., 0.,
            -0.5,  0.5, -0.5,  1.0, 1.0,  0., -1., 0.,
            -0.5, -0.5, -0.5,  0.0, 1.0,  0., -1., 0.,
            -0.5, -0.5, -0.5,  0.0, 1.0,  0., -1., 0.,
            -0.5, -0.5,  0.5,  0.0, 0.0,  0., -1., 0.,
            -0.5,  0.5,  0.5,  1.0, 0.0,  0., -1., 0.,

             0.5,  0.5,  0.5,  1.0, 0.0,  1., 0., 0.,
             0.5,  0.5, -0.5,  1.0, 1.0,  1., 0., 0.,
             0.5, -0.5, -0.5,  0.0, 1.0,  1., 0., 0.,
             0.5, -0.5, -0.5,  0.0, 1.0,  1., 0., 0.,
             0.5, -0.5,  0.5,  0.0, 0.0,  1., 0., 0.,
             0.5,  0.5,  0.5,  1.0, 0.0,  1., 0., 0.,

            -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,
             0.5, -0.5, -0.5,  1.0, 1.0,  -1., 0., 0.,
             0.5, -0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,
             0.5, -0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,
            -0.5, -0.5,  0.5,  0.0, 0.0,  -1., 0., 0.,
            -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,

            -0.5,  0.5, -0.5,  0.0, 1.0,  0., 0., 1.,
             0.5,  0.5, -0.5,  1.0, 1.0,  0., 0., 1.,
             0.5,  0.5,  0.5,  1.0, 0.0,  0., 0., 1.,
             0.5,  0.5,  0.5,  1.0, 0.0,  0., 0., 1.,
            -0.5,  0.5,  0.5,  0.0, 0.0,  0., 0., 1.,
            -0.5,  0.5, -0.5,  0.0, 1.0,  0., 0., 1.,
        ];
        let lighting_cube_vert_bindings = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&lighting_cube_verts),
            label: c"square texture verts".as_ptr(),
            ..Default::default()
        });
        let lighting_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::lighting_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            cull_mode: gfx::CullMode::None,
            depth: gfx::DepthState {
                compare: gfx::CompareFunc::LessEqual,
                write_enabled: true,
                ..Default::default()
            },
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shaders::ATTR_LIGHTING_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_LIGHTING_V_TEX_POS].format = gfx::VertexFormat::Float2;
                layout.attrs[shaders::ATTR_LIGHTING_V_NORMAL].format = gfx::VertexFormat::Float3;
                layout
            },
            label: c"texture draw pipeline".as_ptr(),
            ..Default::default()
        });
        let lit_object = RenderObject {
            mesh: Mesh {
                vertex_buffer: lighting_cube_vert_bindings,
                index_buffer: None,
                element_count: cube_verts.len() / 5,
            },
            transform: glm::Mat4::from_translation(glm::Vec3::new(3., 1.5, 3.)),
            pipeline: lighting_pipeline,
            material: Material::Texture { texture },
        };
        self.objects.push(lit_object);

        let solid_color_pipeline = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::solid_color_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            cull_mode: gfx::CullMode::None,
            depth: gfx::DepthState {
                compare: gfx::CompareFunc::LessEqual,
                write_enabled: true,
                ..Default::default()
            },
            layout: {
                let mut layout = gfx::VertexLayoutState::new();
                layout.attrs[shaders::ATTR_TEXTURE_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_TEXTURE_V_TEX_POS].format = gfx::VertexFormat::Float2;
                layout
            },
            label: c"solid color draw pipeline".as_ptr(),
            ..Default::default()
        });
        let light_obj = RenderObject {
            mesh: Mesh {
                vertex_buffer: cube_vert_bindings,
                index_buffer: None,
                element_count: cube_verts.len() / 5,
            },
            transform: glm::Mat4::from_translation(glm::Vec3::new(5., 3., 5.))
                * glm::Mat4::from_scale(glm::Vec3::new(0.2, 0.2, 0.2)),
            pipeline: solid_color_pipeline,
            material: Material::SolidColor {
                color: glm::Vec4::new(1., 1., 1., 1.),
            },
        };
        self.objects.push(light_obj);

        let sampler = gfx::make_sampler(&gfx::SamplerDesc {
            min_filter: gfx::Filter::Nearest,
            mag_filter: gfx::Filter::Nearest,
            ..Default::default()
        });
        self.sampler = sampler;
        let color_pass_action = gfx::ColorAttachmentAction {
            load_action: gfx::LoadAction::Clear,
            clear_value: BACKGROUND,
            ..Default::default()
        };
        self.pass_action.colors[0] = color_pass_action;
    }

    fn callback_event(&mut self, event: &sap::Event) {
        self.inputs.get_inputs(event);
    }

    fn callback_frame(&mut self) {
        self.metrics.update();
        self.camera
            .update(&mut self.inputs, self.metrics.frame_time);

        gfx::begin_pass(&gfx::Pass {
            action: self.pass_action,
            swapchain: glue::swapchain(),
            ..Default::default()
        });

        gfx::apply_viewport(0, 0, sap::width(), sap::height(), false);

        let projection = self.camera.projection_matrix();
        let view = self.camera.view_matrix();

        for object in &self.objects {
            self.bindings = gfx::Bindings::new();

            self.bindings.vertex_buffers[0] = object.mesh.vertex_buffer;
            if let Some(index) = object.mesh.index_buffer {
                self.bindings.index_buffer = index;
            }

            gfx::apply_pipeline(object.pipeline);

            match object.material {
                Material::Simple => {}
                Material::Texture { texture } => {
                    self.bindings.images[shaders::IMG_TEX] = texture;
                    self.bindings.samplers[shaders::SMP_SAMP] = self.sampler;
                }
                Material::SolidColor { color } => {
                    gfx::apply_uniforms(shaders::UB_SOLID_PARAMS, &gfx::value_as_range(&color));
                }
            }
            gfx::apply_bindings(&self.bindings);

            let vs_params = [object.transform, view, projection];
            gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::slice_as_range(&vs_params));

            gfx::draw(0, object.mesh.element_count, 1);
        }

        gfx::end_pass();
        gfx::commit();
    }
}

fn load_texture(path: &str) -> gfx::Image {
    let image = image::open(path)
        .expect("error reading in texture")
        .flipv()
        .to_rgba8();
    let (width, height) = image.dimensions();
    let image_data = image.into_raw();

    gfx::make_image(&gfx::ImageDesc {
        width: width as i32,
        height: height as i32,
        pixel_format: gfx::PixelFormat::Rgba8,
        data: {
            let mut subimage = gfx::ImageData::new();
            subimage.subimage[0][0] = gfx::slice_as_range(&image_data);
            subimage
        },
        label: c"loaded texture".as_ptr(),
        ..Default::default()
    })
}
