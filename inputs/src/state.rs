use std::f32::consts::PI;
use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::glue;
use sokol::log;
use sokol::time;

use glam as glm;

use crate::BACKGROUND;
use crate::HEIGHT;
use crate::WIDTH;
use crate::shaders;
use crate::temp_util_loc;

#[derive(Default)]
pub struct State {
    pub entities: Vec<Entity>,

    pub renderer: Renderer,

    pub camera: Camera,
    pub inputs: Inputs,

    pub last_frame_time: u64,
    pub current_time: f32,
    pub delta_time: f32,
    pub fps: f32,
}

#[derive(Default)]
pub struct Renderer {
    pub sampler: gfx::Sampler,
    pub pipeline_untextured: gfx::Pipeline,
    pub pipeline_textured: gfx::Pipeline,
    pub bindings: gfx::Bindings,
    pub pass_action: gfx::PassAction,
}

impl State {
    pub fn new() -> State {
        State {
            camera: Camera::new(),
            ..Default::default()
        }
    }

    pub fn update_camera(&mut self) {
        let dmouse = self.inputs.mouse_delta / 200. * self.inputs.mouse_sensitivity;
        self.camera.yaw += dmouse.x;
        self.camera.pitch -= dmouse.y;
        self.camera.pitch = self.camera.pitch.clamp(-PI / 2. + 0.5, PI / 2. - 0.5);
        self.inputs.mouse_delta = glm::Vec2::ZERO;
        self.camera.update_vectors();

        let speed = self.inputs.move_speed * self.delta_time;
        let right = self.camera.right;
        let forward = self.camera.world_up.cross(right);

        let mut movement = glm::Vec3::ZERO;
        if self.inputs.key_pressed[sap::Keycode::W as usize] {
            movement += forward;
        }
        if self.inputs.key_pressed[sap::Keycode::S as usize] {
            movement -= forward;
        }
        if self.inputs.key_pressed[sap::Keycode::A as usize] {
            movement -= right;
        }
        if self.inputs.key_pressed[sap::Keycode::D as usize] {
            movement += right;
        }
        if self.inputs.key_pressed[sap::Keycode::R as usize] {
            movement += self.camera.world_up;
        }
        if self.inputs.key_pressed[sap::Keycode::F as usize] {
            movement -= self.camera.world_up;
        }
        movement = glm::Vec3::normalize_or_zero(movement);
        self.camera.position += movement * speed;

        if self.inputs.key_pressed[sap::Keycode::L as usize] {
            sap::lock_mouse(!sap::mouse_locked());
        }
        if self.inputs.key_pressed[sap::Keycode::Escape as usize] {
            sap::request_quit();
        }

        if self.inputs.key_pressed[sap::Keycode::Equal as usize] {
            self.camera.fov += 0.02;
        }
        if self.inputs.key_pressed[sap::Keycode::Minus as usize] {
            self.camera.fov -= 0.02;
        }

        if self.inputs.major_change {
            self.inputs.major_change = false;
            self.camera.aspect_ratio = sap::widthf() / sap::heightf();
        }
    }

    pub fn update_metrics(&mut self) {
        let current_time = time::now();
        self.delta_time = time::sec(current_time - self.last_frame_time) as f32;
        self.last_frame_time = current_time;
        self.current_time += self.delta_time;
        self.fps = 1.0 / self.delta_time;
    }

    pub fn callback_init(&mut self, user_data: *mut c_void) {
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
            -0.5, 1.5, 5.,     1., 0.7, 0.,
            0.5 , 1.5, 5.,     0., 1., 0.7,
            0.  , 0.5 , 5.,     0.7, 0., 1.,
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
            glm::Vec3::new(0., 0., 0.),
        ];

        let cube_buffer = gfx::make_buffer(&gfx::BufferDesc {
            data: gfx::slice_as_range(&CUBE_VERTS),
            ..Default::default()
        });
        let texture = temp_util_loc::generate_texture("textures/ground.jpg");
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
            self.entities.push(entity);
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
        self.entities.push(tri_entity);

        self.renderer.sampler = gfx::make_sampler(&gfx::SamplerDesc {
            min_filter: gfx::Filter::Nearest,
            mag_filter: gfx::Filter::Nearest,
            ..Default::default()
        });

        self.renderer.pipeline_untextured = gfx::make_pipeline(&gfx::PipelineDesc {
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

        self.renderer.pipeline_textured = gfx::make_pipeline(&gfx::PipelineDesc {
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

        self.renderer.pass_action.colors[0] = gfx::ColorAttachmentAction {
            load_action: gfx::LoadAction::Clear,
            #[rustfmt::skip]
            clear_value: BACKGROUND,
            ..Default::default()
        };
    }

    pub fn callback_frame(&mut self) {
        self.update_metrics();
        self.update_camera();

        gfx::begin_pass(&gfx::Pass {
            action: self.renderer.pass_action,
            swapchain: glue::swapchain(),
            ..Default::default()
        });

        gfx::apply_viewport(15, 15, sap::width() - 30, sap::height() - 30, false);

        let projection = self.camera.projection_matrix();
        let view = self.camera.view_matrix();

        for &entity in &self.entities {
            let model = glm::Mat4::from_scale_rotation_translation(
                entity.scale,
                entity.rotation,
                entity.position,
            );
            let vs_params = [model, view, projection];
            let fs_params = shaders::FsParams { time: self.current_time, _pad_4: [0; 12] };

            if let Some(texture) = entity.texture {
                self.renderer.bindings.vertex_buffers[0] = entity.render_object.vertex_buffer;
                gfx::apply_pipeline(self.renderer.pipeline_textured);
                self.renderer.bindings.images[0] = texture;
                self.renderer.bindings.samplers[0] = self.renderer.sampler;
                gfx::apply_bindings(&self.renderer.bindings);
                gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::value_as_range(&vs_params));
                gfx::apply_uniforms(shaders::UB_FS_PARAMS, &gfx::value_as_range(&fs_params));
            } else {
                self.renderer.bindings.vertex_buffers[0] = entity.render_object.vertex_buffer;
                gfx::apply_pipeline(self.renderer.pipeline_untextured);
                gfx::apply_bindings(&self.renderer.bindings);
                gfx::apply_uniforms(shaders::UB_VS_PARAMS, &gfx::value_as_range(&vs_params));
            }
            gfx::draw(0, entity.render_object.vertex_count, 1);
        }

        gfx::end_pass();
        gfx::commit();
    }

    pub fn callback_event(&mut self, event: &sap::Event) {
        self.inputs.get_inputs(event);
    }
}

#[derive(Default)]
pub struct Camera {
    pub position: glm::Vec3,

    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: glm::Vec3::new(0., 0., -10.),
            up: glm::Vec3::new(0., 1., 0.),
            yaw: PI / 2.,
            front: glm::Vec3::new(0., 0., -1.),
            world_up: glm::Vec3::new(0., 1., 0.),
            fov: 55f32.to_radians(),
            aspect_ratio: WIDTH as f32 / HEIGHT as f32,
            near: 0.1,
            far: 100.,
            ..Default::default()
        }
    }

    pub fn update_vectors(&mut self) {
        let front = glm::Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        self.front = front.normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn projection_matrix(&self) -> glm::Mat4 {
        glm::Mat4::perspective_rh_gl(self.fov, self.aspect_ratio, self.near, self.far)
    }
}

pub struct Inputs {
    pub key_pressed: [bool; 372],
    pub mouse_delta: glm::Vec2,
    pub mouse_sensitivity: f32,
    pub move_speed: f32,
    pub major_change: bool,
    pub click: bool,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            key_pressed: [false; 372],
            mouse_sensitivity: 0.25,
            move_speed: 2.5,
            mouse_delta: glm::Vec2::ZERO,
            major_change: false,
            click: false,
        }
    }

    pub fn get_inputs(&mut self, event: &sap::Event) {
        match event._type {
            sap::EventType::KeyDown => {
                let key = event.key_code as usize;
                self.key_pressed[key] = true;
            }
            sap::EventType::KeyUp => {
                let key = event.key_code as usize;
                self.key_pressed[key] = false;
            }
            sap::EventType::MouseMove => {
                self.mouse_delta += glm::Vec2::new(event.mouse_dx, event.mouse_dy);
            }
            sap::EventType::Resized => {
                self.major_change = true;
            }
            sap::EventType::MouseDown => {
                self.click = true;
            }
            sap::EventType::MouseUp => {
                self.click = false;
            }
            _ => {}
        }
    }
}

impl Default for Inputs {
    fn default() -> Inputs {
        Inputs::new()
    }
}

#[derive(Default, Clone, Copy)]
pub struct RenderObject {
    pub vertex_buffer: gfx::Buffer,
    pub vertex_count: usize,
}

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub render_object: RenderObject,
    pub position: glm::Vec3,
    pub rotation: glm::Quat,
    pub scale: glm::Vec3,
    pub texture: Option<gfx::Image>,
}
