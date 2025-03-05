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
