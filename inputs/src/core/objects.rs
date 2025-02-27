use sokol::gfx;

pub struct RenderObject {
    pub vertex_buffer: gfx::Buffer,
    pub vertex_count: usize,
    pub texture: Option<gfx::Sampler>,
}
