/*
 *  Public functions
 */

mod striking_bonuses;
use striking_bonuses::*;

#[derive(Debug, Default)]
pub struct Game {
    score: u16,
    total_rolls: u8,
    current_frame: u8,
    remaining_rolls_in_frame: u8,
    frame_scores: Vec<u8>,
    sparing: bool,
    sparing_bonus_roll: bool,
    striking_rolls: StrikingBonus,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_frame: 1,
            remaining_rolls_in_frame: 2,
            striking_rolls: new_strike_rolls(),
            frame_scores: vec![],
            ..Default::default()
        }
    }

    pub fn closed(&self) -> bool {
        self.current_frame == 10
            && self.remaining_rolls_in_frame == 0
            && !self.sparing
            && !self.sparing_bonus_roll
            && striking_rolls_are_over(&self.striking_rolls)
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

        if !is_a_bonus_roll && self.is_second_roll_in_frame() && self.pins_overload(pins) {
            // two rolls sum is greater than 10
            return false;
        }

        self.total_rolls = self.total_rolls + 1;
        self.frame_scores.push(pins);

        if !is_a_bonus_roll {
            self.add_score(pins);
        }

        if self.sparing {
            self.add_score(pins);
        }

        let striking_rolls_bonus = get_striking_rolls_bonus(&self.striking_rolls);
        if striking_rolls_bonus > 0 {
            self.add_score(pins * striking_rolls_bonus as u8);
            decrement_striking_rolls_bonus(&mut self.striking_rolls);
        }

        if self.is_first_roll_in_frame() && is_strike(pins) {
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
        self.score = self.score + (pins as u16);
    }

    fn last_frame(&self) -> bool {
        self.current_frame == 10
    }

    fn last_frame_bonus(&self) -> bool {
        self.last_frame()
            && self.remaining_rolls_in_frame == 0
            && has_striking_rolls(&self.striking_rolls)
    }

    fn decrement_rolls_in_frame(&mut self) {
        if self.remaining_rolls_in_frame > 0 {
            self.remaining_rolls_in_frame = self.remaining_rolls_in_frame - 1;
        }
    }

    fn rolls_in_frame_are_over(&self) -> bool {
        self.remaining_rolls_in_frame == 0
    }

    fn set_to_next_frame(&mut self) {
        self.remaining_rolls_in_frame = 2;
        self.current_frame = self.current_frame + 1;
        self.frame_scores = vec![];
    }

    fn update_frame_after_roll(&mut self, pins: u8) {
        self.decrement_rolls_in_frame();
        if !self.last_frame() && (is_strike(pins) || self.rolls_in_frame_are_over()) {
            self.set_to_next_frame();
        }
    }

    fn is_first_roll_in_frame(&self) -> bool {
        self.remaining_rolls_in_frame == 2
    }

    fn is_second_roll_in_frame(&self) -> bool {
        self.remaining_rolls_in_frame == 1
    }

    fn frame_score(&self) -> u8 {
        self.frame_scores.iter().sum()
    }

    fn is_full_score(&self) -> bool {
        self.frame_score() == 10
    }

    fn update_sparing(&mut self) {
        self.sparing_bonus_roll =
            self.last_frame() && self.is_second_roll_in_frame() && self.is_full_score();
        self.sparing = self.is_second_roll_in_frame() && self.is_full_score() && !self.last_frame();
    }

    fn pins_overload(&self, pins: u8) -> bool {
        (self.frame_score() + pins) > 10
    }

    fn add_striking(&mut self) {
        increment_striking_rolls_bonus(&mut self.striking_rolls);
    }
}

fn is_strike(pins: u8) -> bool {
    pins == 10
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
        let game = Game::new();

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 2);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.sparing_bonus_roll, false);
        assert_eq!(game.sparing, false);
        assert_eq!(striking_rolls_are_over(&game.striking_rolls), true);
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
        assert_eq!(game.sparing, true);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn considering_spare() {
        let rolls: Vec<u8> = vec![5, 5, 5, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 21);
        assert_eq!(game.sparing, false);
        assert_eq!(game.frame_scores, vec![]);
        assert_eq!(game.closed(), false);
    }

    #[test]
    fn recognize_not_spare() {
        let rolls: Vec<u8> = vec![5, 4];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.sparing, false);
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
        let mut game = Game::new();
        for pins in rolls {
            game.roll(*pins);
        }
        game
    }
}
