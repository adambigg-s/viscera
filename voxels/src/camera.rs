use std::f32::consts::PI;

use glam as glm;

use sokol::app as sap;

use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Camera {
    pub position: glm::Vec3,

    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub mouse_sensitivity: f32,
    pub move_speed: f32,

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

            front: glm::Vec3::new(0., 0., -1.),
            world_up: glm::Vec3::new(0., 1., 0.),
            up: glm::Vec3::new(0., 1., 0.),
            yaw: PI / 2.,

            mouse_sensitivity: 0.25,
            move_speed: 2.5,

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

    pub fn update(&mut self, inputs: &mut Inputs, delta_time: f32) {
        let dmouse = inputs.mouse_delta / 200. * self.mouse_sensitivity;
        self.yaw += dmouse.x;
        self.pitch -= dmouse.y;
        self.pitch = self.pitch.clamp(-PI / 2. + 0.5, PI / 2. - 0.5);
        inputs.mouse_delta = glm::Vec2::ZERO;
        self.update_vectors();

        let speed = self.move_speed * delta_time;
        let right = self.right;
        let forward = self.world_up.cross(right);
        let mut movement = glm::Vec3::ZERO;
        if inputs.keys_active[sap::Keycode::W as usize] {
            movement += forward;
        }
        if inputs.keys_active[sap::Keycode::S as usize] {
            movement -= forward;
        }
        if inputs.keys_active[sap::Keycode::A as usize] {
            movement -= right;
        }
        if inputs.keys_active[sap::Keycode::D as usize] {
            movement += right;
        }
        if inputs.keys_active[sap::Keycode::R as usize] {
            movement += self.world_up;
        }
        if inputs.keys_active[sap::Keycode::F as usize] {
            movement -= self.world_up;
        }
        movement = glm::Vec3::normalize_or_zero(movement);
        self.position += movement * speed;

        if inputs.keys_active[sap::Keycode::L as usize] {
            sap::lock_mouse(!sap::mouse_locked());
        }
        if inputs.keys_active[sap::Keycode::O as usize] {
            sap::toggle_fullscreen();
        }
        if inputs.keys_active[sap::Keycode::Escape as usize] {
            sap::request_quit();
        }

        if inputs.keys_active[sap::Keycode::Equal as usize] {
            self.fov += 0.02;
        }
        if inputs.keys_active[sap::Keycode::Minus as usize] {
            self.fov -= 0.02;
        }

        if inputs.window_event {
            inputs.window_event = false;
            self.aspect_ratio = sap::widthf() / sap::heightf();
        }
    }
}

pub struct Inputs {
    pub keys_active: [bool; 372],
    pub mouse_click: bool,
    pub mouse_delta: glm::Vec2,
    pub window_event: bool,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys_active: [false; 372],
            mouse_delta: glm::Vec2::ZERO,
            window_event: false,
            mouse_click: false,
        }
    }

    pub fn get_inputs(&mut self, event: &sap::Event) {
        match event._type {
            sap::EventType::KeyDown => {
                let key = event.key_code as usize;
                self.keys_active[key] = true;
            }
            sap::EventType::KeyUp => {
                let key = event.key_code as usize;
                self.keys_active[key] = false;
            }
            sap::EventType::MouseMove => {
                self.mouse_delta += glm::Vec2::new(event.mouse_dx, event.mouse_dy);
            }
            sap::EventType::Resized => {
                self.window_event = true;
            }
            sap::EventType::MouseDown => {
                self.mouse_click = true;
            }
            sap::EventType::MouseUp => {
                self.mouse_click = false;
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
