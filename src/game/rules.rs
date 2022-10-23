static DEFAULT_ROLLS_PER_FRAME: u8 = 2;
static DEFAULT_MAX_FRAMES: u8 = 10;
static DEFAULT_INITIAL_PINS: u8 = 10;
static DEFAULT_PINS_INCREMENT_PER_FRAME: u8 = 0;

#[derive(Debug, Default)]
pub struct Rules {
    pub rolls_per_frame: u8,
    pub max_frames: u8,
    pub initial_pins: u8,
    pub pins_increment_per_frame: u8,
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            rolls_per_frame: DEFAULT_ROLLS_PER_FRAME,
            max_frames: DEFAULT_MAX_FRAMES,
            initial_pins: DEFAULT_INITIAL_PINS,
            pins_increment_per_frame: DEFAULT_PINS_INCREMENT_PER_FRAME,
        }
    }
}
