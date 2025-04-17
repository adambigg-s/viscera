use std::f32::consts::PI;

use glam as glm;

use sokol::app as sap;

use crate::{HEIGHT, WIDTH};

#[derive(Default)]
pub struct Camera {
    pub position: glm::Vec3,
    pub velocity: glm::Vec3,
    pub acceleration: f32,

    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub mouse_sensitivity: f32,
    pub move_speed: f32,

    pub yaw: f32,
    pub pitch: f32,

    pub fov: f32,
    pub default_fov: f32,
    pub zoom_fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: glm::Vec3::new(0., 0., -10.),
            velocity: glm::Vec3::ZERO,
            acceleration: 30.,

            front: glm::Vec3::new(0., 0., -1.),
            world_up: glm::Vec3::new(0., 1., 0.),
            up: glm::Vec3::new(0., 1., 0.),
            yaw: PI / 2.,

            mouse_sensitivity: 0.25,
            move_speed: 2.5,

            fov: 55f32.to_radians(),
            default_fov: 55f32.to_radians(),
            zoom_fov: 40f32.to_radians(),
            aspect_ratio: WIDTH as f32 / HEIGHT as f32,
            near: 0.1,
            far: 100.,

            ..Default::default()
        }
    }

    pub fn update_vectors(&mut self) {
        self.front = glm::Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
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
        self.update_rotation(inputs);
        self.update_movement(inputs, delta_time);
        self.handle_special_keys(inputs);
        self.handle_fov_events(inputs);
    }

    fn handle_special_keys(&mut self, inputs: &mut Inputs) {
        if inputs.keys_active[sap::Keycode::L as usize]
            && !inputs.is_key_processed(sap::Keycode::L as usize)
        {
            sap::lock_mouse(!sap::mouse_locked());
            inputs.set_key_processed(sap::Keycode::L as usize, true);
        }
        if inputs.keys_active[sap::Keycode::O as usize]
            && !inputs.is_key_processed(sap::Keycode::O as usize)
        {
            sap::toggle_fullscreen();
            inputs.set_key_processed(sap::Keycode::O as usize, true);
        }
        if inputs.keys_active[sap::Keycode::Escape as usize] {
            sap::request_quit();
        }

        if inputs.window_event {
            inputs.window_event = false;
            self.aspect_ratio = sap::widthf() / sap::heightf();
        }
    }

    fn handle_fov_events(&mut self, inputs: &mut Inputs) {
        if inputs.mouse_right {
            self.fov = self.zoom_fov;
        } else {
            self.fov = self.default_fov;
        }
    }

    fn update_movement(&mut self, inputs: &mut Inputs, delta_time: f32) {
        let right = self.right;
        let forward = self.world_up.cross(right);
        let mut desired_movement = glm::Vec3::ZERO;
        if inputs.keys_active[sap::Keycode::W as usize] {
            desired_movement += forward;
        }
        if inputs.keys_active[sap::Keycode::S as usize] {
            desired_movement -= forward;
        }
        if inputs.keys_active[sap::Keycode::A as usize] {
            desired_movement -= right;
        }
        if inputs.keys_active[sap::Keycode::D as usize] {
            desired_movement += right;
        }
        if inputs.keys_active[sap::Keycode::R as usize] {
            desired_movement += self.world_up;
        }
        if inputs.keys_active[sap::Keycode::F as usize] {
            desired_movement -= self.world_up;
        }
        desired_movement = glm::Vec3::normalize_or_zero(desired_movement);
        if desired_movement != glm::Vec3::ZERO {
            let desired_velocity = desired_movement * self.move_speed;
            let velocity_diff = desired_velocity - self.velocity;
            let accel_this_frame = self.acceleration * delta_time;
            if velocity_diff.length() > accel_this_frame {
                self.velocity += velocity_diff.normalize() * accel_this_frame;
            } else {
                self.velocity = desired_velocity;
            }
        } else {
            let speed = self.velocity.length();
            if speed > 0. {
                let decel_this_frame = self.acceleration * delta_time;
                if decel_this_frame >= speed {
                    self.velocity = glm::Vec3::ZERO;
                } else {
                    self.velocity -= (self.velocity / speed) * decel_this_frame;
                }
            }
        }
        self.position += self.velocity * delta_time;
    }

    fn update_rotation(&mut self, inputs: &mut Inputs) {
        let dmouse = inputs.mouse_delta / 200. * self.mouse_sensitivity;
        self.yaw += dmouse.x;
        self.pitch -= dmouse.y;
        self.pitch = self.pitch.clamp(-PI / 2. + 0.5, PI / 2. - 0.5);
        inputs.mouse_delta = glm::Vec2::ZERO;
        self.update_vectors();
    }
}

pub struct Inputs {
    pub keys_active: [bool; 372],
    pub keys_processed: [bool; 372],
    pub mouse_left: bool,
    pub mouse_right: bool,
    pub mouse_delta: glm::Vec2,
    pub window_event: bool,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys_active: [false; 372],
            keys_processed: [false; 372],
            mouse_delta: glm::Vec2::ZERO,
            mouse_left: false,
            mouse_right: false,
            window_event: false,
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
                self.keys_processed[key] = false;
            }
            sap::EventType::MouseMove => {
                self.mouse_delta += glm::Vec2::new(event.mouse_dx, event.mouse_dy);
            }
            sap::EventType::Resized => {
                self.window_event = true;
            }
            sap::EventType::MouseDown => match event.mouse_button {
                sap::Mousebutton::Left => self.mouse_left = true,
                sap::Mousebutton::Right => self.mouse_right = true,
                _ => {}
            },
            sap::EventType::MouseUp => match event.mouse_button {
                sap::Mousebutton::Left => self.mouse_left = false,
                sap::Mousebutton::Right => self.mouse_right = false,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn set_key_processed(&mut self, key: usize, value: bool) {
        self.keys_processed[key] = value;
    }

    pub fn is_key_processed(&self, key: usize) -> bool {
        self.keys_processed[key]
    }
}

impl Default for Inputs {
    fn default() -> Inputs {
        Inputs::new()
    }
}
