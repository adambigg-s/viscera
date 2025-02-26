use std::f32::consts::PI;

use sokol::app as sap;
use sokol::gfx;
use sokol::time;

use glam as glm;

use crate::camera::Camera;
use crate::camera::Inputs;

#[derive(Default)]
pub struct State {
    pub pipeline: gfx::Pipeline,
    pub bindings: gfx::Bindings,
    pub pass_action: gfx::PassAction,
    pub camera: Camera,
    pub inputs: Inputs,
    pub last_frame_time: u64,
    pub objects: Vec<RenderObject>,
}

impl State {
    pub fn new() -> State {
        State {
            pipeline: gfx::Pipeline::new(),
            bindings: gfx::Bindings::new(),
            pass_action: gfx::PassAction::new(),
            camera: Camera::new(),
            ..Default::default()
        }
    }

    pub fn update_camera(&mut self) {
        let current_time = time::now();
        let dt = time::sec(current_time - self.last_frame_time) as f32;
        self.last_frame_time = current_time;

        let dmouse = self.inputs.mouse_delta / 200. * self.inputs.mouse_sensitivity;
        self.camera.yaw += dmouse.x;
        self.camera.pitch -= dmouse.y;
        self.camera.pitch = self.camera.pitch.clamp(-PI / 2. + 0.5, PI / 2. - 0.5);
        self.inputs.mouse_delta = glm::Vec2::ZERO;
        self.camera.update_vectors();

        let speed = self.inputs.move_speed * dt;
        let right = self.camera.right;
        let front = self.camera.front;

        if self.inputs.key_pressed[sap::Keycode::W as usize] {
            self.camera.position += front * speed;
        }
        if self.inputs.key_pressed[sap::Keycode::S as usize] {
            self.camera.position -= front * speed;
        }
        if self.inputs.key_pressed[sap::Keycode::A as usize] {
            self.camera.position -= right * speed;
        }
        if self.inputs.key_pressed[sap::Keycode::D as usize] {
            self.camera.position += right * speed;
        }

        if self.inputs.key_pressed[sap::Keycode::L as usize] {
            sap::lock_mouse(self.inputs.mouse_locked);
            self.inputs.mouse_locked = !self.inputs.mouse_locked;
        }
        if self.inputs.key_pressed[sap::Keycode::Escape as usize] {
            sap::request_quit();
        }
    }
}

pub struct RenderObject {
    pub vertex_buffer: gfx::Buffer,
    pub vertex_count: usize,
}
