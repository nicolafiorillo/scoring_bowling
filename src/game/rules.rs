static DEFAULT_ROLLS_PER_FRAME: u8 = 2;
static DEFAULT_MAX_FRAMES: u8 = 10;

#[derive(Debug, Default)]
pub struct Rules {
    pub rolls_per_frame: u8,
    pub max_frames: u8,
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            rolls_per_frame: DEFAULT_ROLLS_PER_FRAME,
            max_frames: DEFAULT_MAX_FRAMES,
        }
    }

    pub fn set_rolls_per_frame(&mut self, rolls_per_frame: u8) {
        self.rolls_per_frame = rolls_per_frame;
    }

    pub fn set_max_frames(&mut self, max_frames: u8) {
        self.max_frames = max_frames;
    }
}
