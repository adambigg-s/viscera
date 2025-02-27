use sokol::gfx;

pub fn generate_texture(path: &str) -> gfx::Image {
    let img = image::open(path).expect("failed to read texture");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    gfx::make_image(&gfx::ImageDesc {
        width: width as i32,
        height: height as i32,
        pixel_format: gfx::PixelFormat::Rgba8,
        data: {
            let mut data = gfx::ImageData::new();
            data.subimage[0][0] = gfx::slice_as_range(&rgba);
            data
        },
        ..Default::default()
    })
}
