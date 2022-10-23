static TOTAL_STRIKE_ROLLS: u8 = 2;

#[derive(Debug, Default)]
pub struct StrikingBonus {
    rolls: Vec<u8>,
}

impl StrikingBonus {
    pub fn new() -> StrikingBonus {
        StrikingBonus {
            rolls: vec![0; TOTAL_STRIKE_ROLLS as usize],
        }
    }

    pub fn striking_rolls_are_over(&self) -> bool {
        self.rolls.iter().sum::<u8>() == 0
    }

    pub fn has_striking_rolls(&self) -> bool {
        self.rolls.iter().sum::<u8>() != 0
    }

    pub fn get_striking_rolls_bonus(&self) -> usize {
        self.rolls.iter().filter(|&&x| x > 0).count()
    }

    pub fn decrement_striking_rolls_bonus(&mut self) {
        self.rolls
            .iter_mut()
            .map(|x| *x = if *x > 0 { *x - 1 } else { 0 })
            .count();
    }

    pub fn increment_striking_rolls_bonus(&mut self) {
        if self.rolls[0] == 0 {
            self.rolls[0] = TOTAL_STRIKE_ROLLS;
        } else if self.rolls[1] == 0 {
            self.rolls[1] = TOTAL_STRIKE_ROLLS;
        }
    }
}

pub fn first_slot(striking_rolls: &StrikingBonus) -> u8 {
    striking_rolls.rolls[0]
}
