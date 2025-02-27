use std::f32::consts::PI;

use sokol::app as sap;

use glam as glm;

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
            aspect_ratio: sap::widthf() / sap::heightf(),
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
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            key_pressed: [false; 372],
            mouse_sensitivity: 0.25,
            move_speed: 2.5,
            mouse_delta: glm::Vec2::ZERO,
            major_change: false,
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
            _ => {}
        }
    }
}

impl Default for Inputs {
    fn default() -> Inputs {
        Inputs::new()
    }
}
