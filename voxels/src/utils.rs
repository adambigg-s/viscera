use sokol::gfx;
use sokol::time;

#[derive(Default)]
pub struct Metrics {
    pub frame_time: f32,
    pub current_time: f32,
}

impl Metrics {
    pub fn update(&mut self) {
        let current_time = time::sec(time::now()) as f32;
        self.frame_time = current_time - self.current_time;
        self.current_time = current_time;
    }

    pub fn display(&self) {
        let fps = 1. / self.frame_time;
        println!("\x1b[20Hfps: {}", fps);
    }
}

pub fn load_texture(path: &str) -> gfx::Image {
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

#[allow(dead_code)]
#[repr(C, align(16))]
pub struct AlignOnePlus4<T> {
    _data: T,
    _padding: [u8; 4],
}

#[allow(dead_code)]
impl<T> AlignOnePlus4<T> {
    pub fn new(_data: T) -> AlignOnePlus4<T> {
        AlignOnePlus4 {
            _data,
            _padding: [0o0; 4],
        }
    }
}

#[allow(dead_code)]
#[repr(C, align(16))]
pub struct AlignTwoPlus4<T, D> {
    _data1: T,
    _data2: D,
    _padding: [u8; 4],
}

#[allow(dead_code)]
impl<T, D> AlignTwoPlus4<T, D> {
    pub fn new(_data1: T, _data2: D) -> AlignTwoPlus4<T, D> {
        AlignTwoPlus4 {
            _data1,
            _data2,
            _padding: [0o0; 4],
        }
    }
}

pub fn cube_verts_uv_normal() -> gfx::Buffer {
    #[rustfmt::skip]
    let cube_verts: [f32; 288] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,  0., 0., -1.,
         0.5, -0.5, -0.5,  1.0, 0.0,  0., 0., -1.,
         0.5,  0.5, -0.5,  1.0, 1.0,  0., 0., -1.,
         0.5,  0.5, -0.5,  1.0, 1.0,  0., 0., -1.,
        -0.5,  0.5, -0.5,  0.0, 1.0,  0., 0., -1.,
        -0.5, -0.5, -0.5,  0.0, 0.0,  0., 0., -1.,

        -0.5, -0.5,  0.5,  0.0, 0.0,  0., 0., 1.,
         0.5, -0.5,  0.5,  1.0, 0.0,  0., 0., 1.,
         0.5,  0.5,  0.5,  1.0, 1.0,  0., 0., 1.,
         0.5,  0.5,  0.5,  1.0, 1.0,  0., 0., 1.,
        -0.5,  0.5,  0.5,  0.0, 1.0,  0., 0., 1.,
        -0.5, -0.5,  0.5,  0.0, 0.0,  0., 0., 1.,

        -0.5,  0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,
        -0.5,  0.5, -0.5,  1.0, 1.0,  -1., 0., 0.,
        -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,
        -0.5, -0.5, -0.5,  0.0, 1.0,  -1., 0., 0.,
        -0.5, -0.5,  0.5,  0.0, 0.0,  -1., 0., 0.,
        -0.5,  0.5,  0.5,  1.0, 0.0,  -1., 0., 0.,

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

        -0.5,  0.5, -0.5,  0.0, 1.0,  0., 1., 0.,
         0.5,  0.5, -0.5,  1.0, 1.0,  0., 1., 0.,
         0.5,  0.5,  0.5,  1.0, 0.0,  0., 1., 0.,
         0.5,  0.5,  0.5,  1.0, 0.0,  0., 1., 0.,
        -0.5,  0.5,  0.5,  0.0, 0.0,  0., 1., 0.,
        -0.5,  0.5, -0.5,  0.0, 1.0,  0., 1., 0.,
    ];
    gfx::make_buffer(&gfx::BufferDesc {
        data: gfx::slice_as_range(&cube_verts),
        label: c"square texture verts".as_ptr(),
        ..Default::default()
    })
}

pub fn cube_vertex_uv() -> gfx::Buffer {
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
        -0.5,  0.5, -0.5,  0.0, 1.0,
    ];
    gfx::make_buffer(&gfx::BufferDesc {
        data: gfx::slice_as_range(&cube_verts),
        ..Default::default()
    })
}
