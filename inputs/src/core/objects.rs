use glam as glm;
use sokol::gfx;

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
