static DEFAULT_ROLLS_PER_FRAME: u8 = 2;

#[derive(Debug, Default)]
pub struct Rules {
    pub rolls_per_frame: u8,
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            rolls_per_frame: DEFAULT_ROLLS_PER_FRAME,
        }
    }

    pub fn set_rolls_per_frame(&mut self, rolls_per_frame: u8) {
        self.rolls_per_frame = rolls_per_frame;
    }
}
