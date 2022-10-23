/*
 *  Public functions
 */

mod striking_bonuses;
use striking_bonuses::*;

pub(crate) mod rules; // TODO: pub(crate)???
use rules::Rules;

#[derive(Debug, Default)]
pub struct Game {
    score: u16,
    pins: u8,
    total_rolls: u8,
    current_frame: u8,
    remaining_rolls_in_frame: u8,
    frame_scores: Vec<u8>,
    sparing: u8,
    striking_rolls: StrikingBonus,
    rules: Rules,
}

impl Game {
    pub fn new(rules: Rules) -> Game {
        Game {
            current_frame: 1,
            remaining_rolls_in_frame: rules.rolls_per_frame,
            pins: rules.initial_pins,
            striking_rolls: StrikingBonus::new(),
            frame_scores: vec![],
            rules,
            ..Default::default()
        }
    }

    pub fn closed(&self) -> bool {
        self.last_frame()
            && self.rolls_in_frame_are_over()
            && self.sparing_is_over()
            && self.striking_rolls.striking_rolls_are_over()
    }

    pub fn score(&self) -> u16 {
        self.score
    }

    pub fn roll(&mut self, pins: u8) -> bool {
        if self.closed() {
            panic!("Game already closed.");
        }

        // bonus rolls is only for last frame
        let is_a_bonus_roll = self.last_frame_bonus();

        if !is_a_bonus_roll && self.is_not_first_roll_in_frame() && self.pins_overload(pins) {
            // more rolls sum is greater than max pins game rule
            return false;
        }

        self.total_rolls += 1;
        self.frame_scores.push(pins);

        if !is_a_bonus_roll {
            self.add_score(pins);
        }

        if self.have_sparing() {
            self.add_score(pins);
        }

        let striking_rolls_bonus = self.striking_rolls.get_striking_rolls_bonus();
        if striking_rolls_bonus > 0 {
            self.add_score(pins * striking_rolls_bonus as u8);
            self.striking_rolls.decrement_striking_rolls_bonus();
        }

        if self.is_first_roll_in_frame() && self.is_strike(pins) {
            // strike!
            self.add_striking();
            self.remaining_rolls_in_frame = 0;
        }

        self.update_sparing();
        self.update_frame_after_roll(pins);

        true
    }

    /*
     *  Private functions
     */

    fn add_score(&mut self, pins: u8) {
        self.score += pins as u16;
    }

    fn last_frame(&self) -> bool {
        self.current_frame == self.rules.max_frames
    }

    fn last_frame_bonus(&self) -> bool {
        self.last_frame()
            && self.rolls_in_frame_are_over()
            && (self.striking_rolls.has_striking_rolls() || self.have_sparing())
    }

    fn decrement_rolls_in_frame(&mut self) {
        if self.remaining_rolls_in_frame > 0 {
            self.remaining_rolls_in_frame -= 1;
        }
    }

    fn rolls_in_frame_are_over(&self) -> bool {
        self.remaining_rolls_in_frame == 0
    }

    fn sparing_is_over(&self) -> bool {
        self.sparing == 0
    }

    fn have_sparing(&self) -> bool {
        self.sparing > 0
    }

    fn set_to_next_frame(&mut self) {
        self.remaining_rolls_in_frame = self.rules.rolls_per_frame;
        self.current_frame += 1;
        self.frame_scores = vec![];
        self.pins += self.rules.pins_increment_per_frame;
    }

    fn update_frame_after_roll(&mut self, pins: u8) {
        self.decrement_rolls_in_frame();
        if !self.last_frame() && (self.is_strike(pins) || self.rolls_in_frame_are_over()) {
            self.set_to_next_frame();
        }
    }

    fn is_first_roll_in_frame(&self) -> bool {
        self.remaining_rolls_in_frame == self.rules.rolls_per_frame
    }

    fn is_not_first_roll_in_frame(&self) -> bool {
        self.remaining_rolls_in_frame != self.rules.rolls_per_frame
            && self.remaining_rolls_in_frame != 0
    }

    fn frame_score(&self) -> u8 {
        self.frame_scores.iter().sum()
    }

    fn is_full_score(&self) -> bool {
        self.frame_score() == self.pins
    }

    fn update_sparing(&mut self) {
        if self.sparing > 0 {
            self.sparing -= 1
        }

        if self.is_not_first_roll_in_frame() && self.is_full_score() {
            self.sparing += 1
        }
    }

    fn pins_overload(&self, pins: u8) -> bool {
        (self.frame_score() + pins) > self.pins
    }

    fn add_striking(&mut self) {
        self.striking_rolls.increment_striking_rolls_bonus();
    }

    fn is_strike(&self, pins: u8) -> bool {
        pins == self.pins
    }
}

/*
 *  Tests
 */

#[cfg(test)]
mod normal_game {
    use crate::game::striking_bonuses::*;
    use crate::game::*;

    #[test]
    fn initial_status_of_game() {
        let game = Game::new(Rules::new());

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.pins, 10);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 2);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.striking_rolls.striking_rolls_are_over(), true);
        assert_eq!(game.rules.rolls_per_frame, 2);
        assert_eq!(game.rules.max_frames, 10);
    }

    #[test]
    fn correct_rolls() {
        let rolls: Vec<u8> = vec![4, 4];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 8);
        assert_eq!(game.current_frame, 2);
        assert_eq!(game.remaining_rolls_in_frame, 2);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn incorrect_rolls() {
        let rolls: Vec<u8> = vec![9, 9];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 1);
        assert_eq!(game.closed(), false);
    }
    #[test]

    fn the_wrost_game() {
        let rolls: Vec<u8> = vec![0; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 0);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_wrost_game_but_not_finished_yet() {
        let rolls: Vec<u8> = vec![0; 19];
        let game = play_this_game(&rolls);

        assert_eq!(game.closed(), false);
    }

    #[test]
    fn the_perfect_game() {
        let rolls: Vec<u8> = vec![10; 12];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 300);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_almost_perfect_game() {
        let rolls: Vec<u8> = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 270);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn a_normal_game() {
        let rolls: Vec<u8> = vec![1; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 20);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn recognize_spare() {
        let rolls: Vec<u8> = vec![5, 5];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 10);
        assert_eq!(game.sparing, 1);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn considering_spare() {
        let rolls: Vec<u8> = vec![5, 5, 5, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 21);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn recognize_not_spare() {
        let rolls: Vec<u8> = vec![5, 4];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn recognize_strike() {
        let rolls: Vec<u8> = vec![10];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 10);
        assert_eq!(first_slot(&game.striking_rolls), 2);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn considering_simple_strike() {
        let rolls: Vec<u8> = vec![10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 14);
        assert_eq!(first_slot(&game.striking_rolls), 0);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn go_to_next_frame_after_strike() {
        let rolls: Vec<u8> = vec![10];
        let game = play_this_game(&rolls);

        assert_eq!(game.current_frame, 2);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn a_spare_and_a_strike() {
        let rolls: Vec<u8> = vec![6, 4, 10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 34);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn a_strike_and_a_spare() {
        let rolls: Vec<u8> = vec![10, 6, 4, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 33);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn three_spares() {
        let rolls: Vec<u8> = vec![5; 7];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 50);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn nine_spares() {
        let rolls: Vec<u8> = vec![5; 19];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 140);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn ten_spares_without_last_roll() {
        let rolls: Vec<u8> = vec![5; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 145);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn all_spares() {
        let rolls: Vec<u8> = vec![5; 21];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 150);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn two_strike() {
        let rolls: Vec<u8> = vec![10, 10];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 30);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn one_strike_and_a_open() {
        let rolls: Vec<u8> = vec![10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 14);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn two_strike_and_a_open() {
        let rolls: Vec<u8> = vec![10, 10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 35);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn three_strike() {
        let rolls: Vec<u8> = vec![10; 3];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 60);
        assert_eq!(game.closed(), false);
    }

    /*
     *  Other random examples
     */

    #[test]
    fn example_1() {
        let rolls: Vec<u8> = vec![1, 4, 4, 5, 6, 4, 5, 5, 10, 0, 1, 7, 3, 6, 4, 10, 2, 8, 6];
        test_example(133, &rolls);
    }

    #[test]
    fn example_2() {
        let rolls: Vec<u8> = vec![9, 0, 9, 0, 9, 0, 9, 0, 9, 0, 9, 0, 9, 0, 9, 0, 9, 0, 9, 0];
        test_example(90, &rolls);
    }

    #[test]
    fn example_3() {
        let rolls: Vec<u8> = vec![5; 21];
        test_example(150, &rolls);
    }

    #[test]
    fn example_4() {
        let rolls: Vec<u8> = vec![7, 2, 9, 0, 9, 0, 9, 0, 9, 1, 9, 1, 10, 10, 8, 0, 7, 1];
        test_example(137, &rolls);
    }

    #[test]
    fn example_5() {
        let rolls: Vec<u8> = vec![0, 6, 9, 1, 10, 6, 3, 7, 1, 9, 0, 5, 5, 10, 7, 3, 7, 0];
        test_example(135, &rolls);
    }

    #[test]
    fn example_6() {
        let rolls: Vec<u8> = vec![
            1, 9, 0, 9, 9, 0, 6, 0, 1, 5, 7, 0, 8, 2, 7, 0, 9, 0, 10, 10, 0,
        ];
        test_example(100, &rolls);
    }

    #[test]
    fn example_7() {
        let rolls: Vec<u8> = vec![9, 1, 9, 0, 8, 0, 0, 10, 7, 2, 0, 4, 8, 1, 0, 6, 3, 0, 8, 0];
        test_example(92, &rolls);
    }

    #[test]
    fn example_8() {
        let rolls: Vec<u8> = vec![3, 5, 7, 0, 3, 7, 0, 3, 3, 3, 10, 9, 1, 8, 2, 6, 2, 9, 1, 6];
        test_example(112, &rolls);
    }

    #[test]
    fn example_9() {
        let rolls: Vec<u8> = vec![8, 2, 10, 7, 1, 7, 1, 8, 0, 9, 0, 10, 10, 9, 1, 10, 7, 0];
        test_example(157, &rolls);
    }

    #[test]
    fn example_10() {
        let rolls: Vec<u8> = vec![9, 1, 8, 2, 10, 7, 3, 5, 3, 3, 4, 8, 2, 0, 7, 8, 2, 0, 10, 8];
        test_example(133, &rolls);
    }

    #[test]
    fn example_11() {
        let rolls: Vec<u8> = vec![10, 10, 6, 0, 10, 10, 0, 9, 7, 0, 7, 0, 9, 1, 9, 1, 7];
        test_example(146, &rolls);
    }

    #[test]
    fn example_12() {
        let rolls: Vec<u8> = vec![9, 1, 10, 10, 7, 3, 6, 3, 9, 1, 10, 9, 1, 9, 1, 9, 0];
        test_example(179, &rolls);
    }

    #[test]
    fn example_13() {
        let rolls: Vec<u8> = vec![3, 4, 9, 0, 3, 0, 0, 9, 5, 5, 10, 3, 0, 0, 1, 10, 0, 7];
        test_example(89, &rolls);
    }

    #[test]
    fn example_14() {
        let rolls: Vec<u8> = vec![
            9, 1, 4, 4, 9, 0, 9, 0, 9, 1, 9, 0, 9, 0, 9, 0, 9, 0, 10, 8, 0,
        ];
        test_example(113, &rolls);
    }

    fn test_example(score: u16, rolls: &Vec<u8>) {
        let game = play_this_game(&rolls);

        assert_eq!(game.score, score);
        assert_eq!(game.closed(), true);
    }

    fn play_this_game(rolls: &Vec<u8>) -> Game {
        let mut game = Game::new(Rules::new());
        for pins in rolls {
            game.roll(*pins);
        }
        game
    }
}

#[cfg(test)]
mod three_rolls_per_frame_game {
    use crate::game::*;

    #[test]
    fn initial_status_of_game() {
        let mut rules = Rules::new();
        rules.rolls_per_frame = 3;

        let game = Game::new(rules);

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 3);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.striking_rolls.striking_rolls_are_over(), true);
        assert_eq!(game.rules.rolls_per_frame, 3);
    }

    #[test]
    fn incorrect_rolls() {
        let rolls: Vec<u8> = vec![4, 4, 4];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 8);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 1);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn correct_rolls() {
        let rolls: Vec<u8> = vec![3, 3, 3];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.current_frame, 2);
        assert_eq!(game.remaining_rolls_in_frame, 3);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn the_wrost_game() {
        let rolls: Vec<u8> = vec![0; 30];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 0);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_wrost_game_but_not_finished_yet() {
        let rolls: Vec<u8> = vec![0; 29];
        let game = play_this_game(&rolls);

        assert_eq!(game.closed(), false);
    }

    #[test]
    fn the_perfect_game() {
        let rolls: Vec<u8> = vec![10; 12];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 300);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_almost_perfect_game() {
        let rolls: Vec<u8> = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 270);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn a_normal_game() {
        let rolls: Vec<u8> = vec![1; 30];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 30);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn recognize_spare() {
        let rolls: Vec<u8> = vec![3, 3, 4];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 10);
        assert_eq!(game.current_frame, 2);
        assert_eq!(game.remaining_rolls_in_frame, 3);
        assert_eq!(game.sparing, 1);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn considering_spare() {
        let rolls: Vec<u8> = vec![2, 2, 6, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 14);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.current_frame, 3);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn recognize_not_spare() {
        let rolls: Vec<u8> = vec![3, 3, 3];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.current_frame, 2);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn recognize_strike() {
        let rolls: Vec<u8> = vec![10];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 10);
        assert_eq!(game.current_frame, 2);
        assert_eq!(first_slot(&game.striking_rolls), 2);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn considering_simple_strike() {
        let rolls: Vec<u8> = vec![10, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 15);
        assert_eq!(game.current_frame, 3);
        assert_eq!(first_slot(&game.striking_rolls), 0);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn go_to_next_frame_after_strike() {
        let rolls: Vec<u8> = vec![10];
        let game = play_this_game(&rolls);

        assert_eq!(game.current_frame, 2);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn a_spare_and_a_strike() {
        let rolls: Vec<u8> = vec![3, 3, 4, 10, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 35);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn a_strike_and_a_spare() {
        let rolls: Vec<u8> = vec![10, 4, 4, 2, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 32);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn three_spares() {
        let rolls: Vec<u8> = vec![3, 3, 4, 4, 4, 2, 6, 1, 3, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 42);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn nine_spares() {
        let rolls: Vec<u8> = vec![
            8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 1,
        ];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 156);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn ten_spares_without_last_roll() {
        let rolls: Vec<u8> = vec![
            8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1,
            1,
        ];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 172);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn all_spares() {
        let rolls: Vec<u8> = vec![
            8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1, 1, 8, 1,
            1, 1,
        ];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 173);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn one_strike_and_a_open() {
        let rolls: Vec<u8> = vec![10, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 15);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn two_strike_and_a_open() {
        let rolls: Vec<u8> = vec![10, 10, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 36);
        assert_eq!(game.closed(), false);
    }

    fn play_this_game(rolls: &Vec<u8>) -> Game {
        let mut rules = Rules::new();
        rules.rolls_per_frame = 3;

        let mut game = Game::new(rules);
        for pins in rolls {
            game.roll(*pins);
        }
        game
    }
}

#[cfg(test)]
mod twelve_frames_game {
    use crate::game::*;

    #[test]
    fn initial_status_of_game() {
        let mut rules = Rules::new();
        rules.max_frames = 12;

        let game = Game::new(rules);

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 2);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.striking_rolls.striking_rolls_are_over(), true);
        assert_eq!(game.rules.rolls_per_frame, 2);
        assert_eq!(game.rules.max_frames, 12);
    }

    #[test]
    fn the_wrost_game() {
        let rolls: Vec<u8> = vec![0; 24];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 0);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_wrost_game_but_not_finished_yet() {
        let rolls: Vec<u8> = vec![0; 23];
        let game = play_this_game(&rolls);

        assert_eq!(game.closed(), false);
    }

    #[test]
    fn the_perfect_game() {
        let rolls: Vec<u8> = vec![10; 14];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 360);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_almost_perfect_game() {
        let rolls: Vec<u8> = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 330);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn a_normal_game() {
        let rolls: Vec<u8> = vec![1; 24];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 24);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn all_spares() {
        let rolls: Vec<u8> = vec![5; 25];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 180);
        assert_eq!(game.closed(), true);
    }

    fn play_this_game(rolls: &Vec<u8>) -> Game {
        let mut rules = Rules::new();
        rules.max_frames = 12;

        let mut game = Game::new(rules);
        for pins in rolls {
            game.roll(*pins);
        }
        game
    }
}

#[cfg(test)]
mod incremental_pins_game {
    use crate::game::*;

    #[test]
    fn initial_status_of_game() {
        let mut rules = Rules::new();
        rules.initial_pins = 1;
        rules.pins_increment_per_frame = 1;

        let game = Game::new(rules);

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.pins, 1);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 2);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.striking_rolls.striking_rolls_are_over(), true);
        assert_eq!(game.rules.rolls_per_frame, 2);
        assert_eq!(game.rules.max_frames, 10);
        assert_eq!(game.rules.initial_pins, 1);
        assert_eq!(game.rules.pins_increment_per_frame, 1);
    }

    #[test]
    fn the_wrost_game() {
        let rolls: Vec<u8> = vec![0; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 0);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn simple_strike() {
        let rolls: Vec<u8> = vec![1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 1);
        assert_eq!(game.current_frame, 2);
        assert_eq!(first_slot(&game.striking_rolls), 2);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn the_perfect_game() {
        let rolls: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 191);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_almost_perfect_game() {
        let rolls: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 161);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn a_normal_game() {
        let rolls: Vec<u8> = vec![1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 24);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn all_spares() {
        let rolls: Vec<u8> = vec![1, 2, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 4, 5, 5, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 86);
        assert_eq!(game.closed(), true);
    }

    fn play_this_game(rolls: &Vec<u8>) -> Game {
        let mut rules = Rules::new();
        rules.initial_pins = 1;
        rules.pins_increment_per_frame = 1;

        let mut game = Game::new(rules);
        for pins in rolls {
            dbg!("{:?}", &game);
            game.roll(*pins);
        }
        game
    }
}

#[cfg(test)]
mod twelve_frames_and_three_rolls_for_frame_game {
    use crate::game::*;

    #[test]
    fn initial_status_of_game() {
        let mut rules = Rules::new();
        rules.max_frames = 12;
        rules.rolls_per_frame = 3;

        let game = Game::new(rules);

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 3);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.sparing, 0);
        assert_eq!(game.striking_rolls.striking_rolls_are_over(), true);
        assert_eq!(game.rules.rolls_per_frame, 3);
        assert_eq!(game.rules.max_frames, 12);
    }

    #[test]
    fn the_wrost_game() {
        let rolls: Vec<u8> = vec![0; 36];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 0);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_wrost_game_but_not_finished_yet() {
        let rolls: Vec<u8> = vec![0; 35];
        let game = play_this_game(&rolls);

        assert_eq!(game.closed(), false);
    }

    #[test]
    fn the_perfect_game() {
        let rolls: Vec<u8> = vec![10; 14];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 360);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn the_almost_perfect_game() {
        let rolls: Vec<u8> = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 330);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn a_normal_game() {
        let rolls: Vec<u8> = vec![1; 36];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 36);
        assert_eq!(game.closed(), true);
    }

    #[test]
    fn all_spares() {
        let rolls: Vec<u8> = vec![
            4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3, 3, 4, 3,
            3, 4, 3, 3, 4, 3, 3, 1,
        ];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 165);
        assert_eq!(game.closed(), true);
    }

    fn play_this_game(rolls: &Vec<u8>) -> Game {
        let mut rules = Rules::new();
        rules.max_frames = 12;
        rules.rolls_per_frame = 3;

        let mut game = Game::new(rules);
        for pins in rolls {
            game.roll(*pins);
        }
        game
    }
}
