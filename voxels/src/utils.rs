use sokol::gfx;
use sokol::time;

#[derive(Default)]
pub struct Metrics {
    pub frame_time: f32,
    pub last_frame_time: u64,
}

impl Metrics {
    pub fn update(&mut self) {
        let current_time = time::now();
        self.frame_time = time::sec(current_time - self.last_frame_time) as f32;
        self.last_frame_time = current_time;
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
