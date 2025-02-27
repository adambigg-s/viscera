use std::f32::consts::PI;

use sokol::app as sap;
use sokol::gfx;
use sokol::time;

use glam as glm;

use crate::camera::Camera;
use crate::camera::Inputs;
use crate::core::objects::Entity;

#[derive(Default)]
pub struct State {
    pub entities: Vec<Entity>,
    pub sampler: gfx::Sampler,

    pub pipeline_untextured: gfx::Pipeline,
    pub pipeline_textured: gfx::Pipeline,
    pub bindings: gfx::Bindings,
    pub pass_action: gfx::PassAction,

    pub camera: Camera,
    pub inputs: Inputs,

    pub last_frame_time: u64,
    pub delta_time: f32,
    pub fps: f32,
}

impl State {
    pub fn new() -> State {
        State {
            pipeline_untextured: gfx::Pipeline::new(),
            bindings: gfx::Bindings::new(),
            pass_action: gfx::PassAction::new(),
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
        self.fps = 1.0 / self.delta_time;
    }
}
