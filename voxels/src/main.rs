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
use utils::*;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1200;
#[rustfmt::skip]
const BACKGROUND: gfx::Color = gfx::Color { r: 0.07, g: 0.07, b: 0.13, a: 1., };

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
        window_title: c"lighting stuff".as_ptr(),
        fullscreen: false,
        high_dpi: true,
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
    LightSource { _coldr: glm::Vec4 },
}

struct Transform {
    position: glm::Vec3,
    rotation: glm::Quat,
    scale: glm::Vec3,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: glm::Vec3::ZERO,
            rotation: glm::Quat::IDENTITY,
            scale: glm::Vec3::ONE,
        }
    }
}

impl Transform {
    fn to_matrix(&self) -> glm::Mat4 {
        glm::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

struct Mesh {
    vertex_buffer: gfx::Buffer,
    index_buffer: Option<gfx::Buffer>,
    element_count: usize,
}

struct RenderObject {
    mesh: Mesh,
    transform: Transform,
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
            -0.5, -0.5, 0., 1., 0., 0.7,
            0.5, -0.5, 0., 0.7, 1., 0.,
            0., 0.5, 0., 0., 0.7, 1.,
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
            label: c"simple draw pipeline with frag interp".as_ptr(),
            ..Default::default()
        });
        for idx in 0..12 {
            let object = RenderObject {
                mesh: Mesh {
                    vertex_buffer: tri_vert_bindings,
                    index_buffer: Some(tri_index_bindings),
                    element_count: tri_indices.len(),
                },
                transform: Transform {
                    position: glm::Vec3::new(idx as f32, 0., idx as f32),
                    ..Default::default()
                },
                pipeline: tri_pipeline,
                material: Material::Simple,
            };
            self.objects.push(object);
        }

        let plain_triangle = gfx::make_pipeline(&gfx::PipelineDesc {
            shader: gfx::make_shader(&shaders::solid_color_shader_desc(gfx::query_backend())),
            primitive_type: gfx::PrimitiveType::Triangles,
            index_type: gfx::IndexType::Uint16,
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
            label: c"solid draw pipeline with vertices".as_ptr(),
            ..Default::default()
        });
        let solid_tri = RenderObject {
            mesh: Mesh {
                vertex_buffer: tri_vert_bindings,
                index_buffer: Some(tri_index_bindings),
                element_count: tri_indices.len(),
            },
            transform: Transform {
                position: glm::Vec3::new(2., 2., 7.),
                ..Default::default()
            },
            pipeline: plain_triangle,
            material: Material::SolidColor {
                color: glm::Vec4::new(0., 1., 1., 1.),
            },
        };
        self.objects.push(solid_tri);

        let texture = load_texture("textures/num_grid.png");
        let lighting_cube_vert_bindings = cube_verts_uv_normal();

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
        for dz in 0..15 {
            for dx in 0..15 {
                let object = RenderObject {
                    mesh: Mesh {
                        vertex_buffer: lighting_cube_vert_bindings,
                        index_buffer: None,
                        element_count: 36,
                    },
                    transform: Transform {
                        position: glm::Vec3::new(dx as f32, -1., dz as f32),
                        ..Default::default()
                    },
                    pipeline: lighting_pipeline,
                    material: Material::Texture { texture },
                };
                self.objects.push(object);
            }
        }
        let lit_object = RenderObject {
            mesh: Mesh {
                vertex_buffer: lighting_cube_vert_bindings,
                index_buffer: None,
                element_count: 36,
            },
            transform: Transform {
                position: glm::Vec3::new(3., 1.5, 3.),
                ..Default::default()
            },
            pipeline: lighting_pipeline,
            material: Material::Texture { texture },
        };
        self.objects.push(lit_object);

        let solid_cube_vertex = cube_vertex_uv();
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
                layout.attrs[shaders::ATTR_SOLID_COLOR_POSITION].format = gfx::VertexFormat::Float3;
                layout.attrs[shaders::ATTR_SOLID_COLOR_V_TEX_POS].format =
                    gfx::VertexFormat::Float2;
                layout
            },
            label: c"solid color draw pipeline".as_ptr(),
            ..Default::default()
        });
        let light_obj = RenderObject {
            mesh: Mesh {
                vertex_buffer: solid_cube_vertex,
                index_buffer: None,
                element_count: 36,
            },
            transform: Transform {
                position: glm::Vec3::new(5., 2., 5.),
                scale: glm::Vec3::splat(0.2),
                ..Default::default()
            },
            pipeline: solid_color_pipeline,
            material: Material::LightSource {
                _coldr: glm::Vec4::new(1., 1., 1., 1.),
            },
        };
        self.objects.push(light_obj);

        let sampler = gfx::make_sampler(&gfx::SamplerDesc {
            min_filter: gfx::Filter::Linear,
            mag_filter: gfx::Filter::Linear,
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
        self.metrics.display();
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

        let light_pos = self.objects.last().unwrap().transform.position;
        let light_color = glm::Vec4::new(
            self.metrics.current_time.sin(),
            self.metrics.current_time.cos(),
            self.metrics.current_time.sin() * self.metrics.current_time.cos(),
            1.,
        );

        for object in &mut self.objects {
            self.bindings = gfx::Bindings::new();

            self.bindings.vertex_buffers[0] = object.mesh.vertex_buffer;
            if let Some(index) = object.mesh.index_buffer {
                self.bindings.index_buffer = index;
            }

            gfx::apply_pipeline(object.pipeline);

            match object.material {
                Material::Simple => {}
                Material::SolidColor { color } => {
                    gfx::apply_uniforms(shaders::UB_SOLID_PARAMS, &gfx::value_as_range(&color));
                }
                Material::LightSource { _coldr } => {
                    gfx::apply_uniforms(
                        shaders::UB_SOLID_PARAMS,
                        &gfx::value_as_range(&light_color),
                    );
                }
                Material::Texture { texture } => {
                    self.bindings.images[shaders::IMG_TEX] = texture;
                    self.bindings.samplers[shaders::SMP_SAMP] = self.sampler;
                    gfx::apply_uniforms(
                        shaders::UB_LIGHTING_PARAMS,
                        &gfx::value_as_range(&shaders::LightingParams {
                            light_color: light_color.to_array(),
                            light_pos: light_pos.to_array(),
                            _pad_28: [0; 4],
                            view_pos: self.camera.position.to_array(),
                            _pad_44: [0; 4],
                        }),
                    );
                }
            }

            gfx::apply_bindings(&self.bindings);

            let vs_params = [object.transform.to_matrix(), view, projection];
            gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::slice_as_range(&vs_params));

            gfx::draw(0, object.mesh.element_count, 1);
        }

        gfx::end_pass();
        gfx::commit();
    }
}
