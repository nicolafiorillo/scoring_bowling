static TOTAL_STRIKE_ROLLS: u8 = 2;

#[derive(Debug, Default)]
pub struct StrikingBonus {
    rolls: Vec<u8>,
}

pub fn new_strike_rolls() -> StrikingBonus {
    StrikingBonus {
        rolls: vec![0; TOTAL_STRIKE_ROLLS as usize],
    }
}

pub fn striking_rolls_are_over(striking_rolls: &StrikingBonus) -> bool {
    striking_rolls.rolls.iter().sum::<u8>() == 0
}

pub fn has_striking_rolls(striking_rolls: &StrikingBonus) -> bool {
    striking_rolls.rolls.iter().sum::<u8>() != 0
}

pub fn get_striking_rolls_bonus(striking_rolls: &StrikingBonus) -> usize {
    striking_rolls.rolls.iter().filter(|&&x| x > 0).count()
}

pub fn decrement_striking_rolls_bonus(striking_rolls: &mut StrikingBonus) {
    striking_rolls
        .rolls
        .iter_mut()
        .map(|x| *x = if *x > 0 { *x - 1 } else { 0 })
        .count();
}

pub fn increment_striking_rolls_bonus(striking_rolls: &mut StrikingBonus) {
    if striking_rolls.rolls[0] == 0 {
        striking_rolls.rolls[0] = TOTAL_STRIKE_ROLLS;
    } else if striking_rolls.rolls[1] == 0 {
        striking_rolls.rolls[1] = TOTAL_STRIKE_ROLLS;
    }
}

pub fn first_slot(striking_rolls: &StrikingBonus) -> u8 {
    striking_rolls.rolls[0]
}
