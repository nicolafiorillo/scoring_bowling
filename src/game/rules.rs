#[derive(Debug, Default)]
pub struct Rules {
    pub rolls_per_frame: u8,
}

impl Rules {
    pub fn new() -> Rules {
        Rules { rolls_per_frame: 2 }
    }
}
