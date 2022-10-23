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

    pub fn set_rolls_per_frame(&mut self, rolls_per_frame: u8) {
        self.rolls_per_frame = rolls_per_frame;
    }

    pub fn set_max_frames(&mut self, max_frames: u8) {
        self.max_frames = max_frames;
    }

    pub fn set_pins(&mut self, pins: (u8, u8)) {
        self.initial_pins = pins.0;
        self.pins_increment_per_frame = pins.1;
    }
}
