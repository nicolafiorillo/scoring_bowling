/*
 *  Public functions
 */

#[derive(Debug, Default)]
pub struct Game {
    score: u16, // max 300 points in game
    total_rolls: u8,
    current_frame: u8,
    remaining_rolls_in_frame: u8,
    frame_score: u8,
    sparing: bool,
    sparing_bonus_roll: bool,
    striking_1: u8,
    striking_2: u8,
}

pub fn new_game() -> Game {
    Game {
        current_frame: 1,
        remaining_rolls_in_frame: 2,
        ..Default::default()
    }
}

pub fn game_closed(game: &Game) -> bool {
    game.current_frame == 10
        && game.remaining_rolls_in_frame == 0
        && !game.sparing
        && !game.sparing_bonus_roll
        && game.striking_1 == 0
        && game.striking_2 == 0
}

pub fn score(game: &Game) -> u16 {
    game.score
}

pub fn roll(game: &mut Game, pins: u8) -> bool {
    if game_closed(game) {
        panic!("Game already closed.");
    }

    // bonus rolls is only for last frame
    let is_a_bonus_roll = last_frame_bonus(game);

    if !is_a_bonus_roll && is_second_roll_in_frame(game) && pins_overload(game, pins) {
        // two rolls sum is greater than 10
        return false;
    }

    game.total_rolls = game.total_rolls + 1;
    game.frame_score = game.frame_score + pins;

    if !is_a_bonus_roll {
        add_score(game, pins);
    }

    if game.sparing {
        add_score(game, pins);
    }

    if game.striking_1 > 0 {
        add_score(game, pins);
        game.striking_1 = game.striking_1 - 1;
    }
    if game.striking_2 > 0 {
        add_score(game, pins);
        game.striking_2 = game.striking_2 - 1;
    }

    if is_first_roll_in_frame(game) && is_strike(pins) {
        // strike!
        add_striking(game);
        game.remaining_rolls_in_frame = 0;
    }

    update_sparing(game);
    update_frame_after_roll(game, pins);

    true
}

/*
 *  Private functions
 */

fn add_score(game: &mut Game, pins: u8) {
    game.score = game.score + (pins as u16);
}

fn last_frame_bonus(game: &mut Game) -> bool {
    last_frame(game)
        && game.remaining_rolls_in_frame == 0
        && (game.striking_1 + game.striking_2) > 0
}

fn is_strike(pins: u8) -> bool {
    pins == 10
}

fn pins_overload(game: &Game, pins: u8) -> bool {
    (game.frame_score + pins) > 10
}

fn add_striking(game: &mut Game) {
    if game.striking_1 == 0 {
        game.striking_1 = 2;
    } else if game.striking_2 == 0 {
        game.striking_2 = 2;
    }
}

fn update_frame_after_roll(game: &mut Game, pins: u8) {
    if game.remaining_rolls_in_frame > 0 {
        game.remaining_rolls_in_frame = game.remaining_rolls_in_frame - 1;
    }

    if !last_frame(game) && (is_strike(pins) || rolls_in_frame_are_over(game)) {
        set_to_next_frame(game);
    }
}

fn rolls_in_frame_are_over(game: &mut Game) -> bool {
    game.remaining_rolls_in_frame == 0
}

fn last_frame(game: &mut Game) -> bool {
    game.current_frame == 10
}

fn set_to_next_frame(game: &mut Game) {
    game.remaining_rolls_in_frame = 2;
    game.current_frame = game.current_frame + 1;
    game.frame_score = 0;
}

fn update_sparing(game: &mut Game) {
    game.sparing_bonus_roll =
        last_frame(game) && is_second_roll_in_frame(game) && game.frame_score == 10;
    game.sparing = is_second_roll_in_frame(game) && game.frame_score == 10 && !last_frame(game);
    if game.sparing {
        game.frame_score = 0;
    }
}

fn is_first_roll_in_frame(game: &mut Game) -> bool {
    game.remaining_rolls_in_frame == 2
}

fn is_second_roll_in_frame(game: &mut Game) -> bool {
    game.remaining_rolls_in_frame == 1
}

/*
 *  Tests
 */

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn initial_status_of_game() {
        let game = new_game();

        assert_eq!(game.score, 0);
        assert_eq!(game.total_rolls, 0);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 2);
        assert_eq!(game.frame_score, 0);
        assert_eq!(game.sparing_bonus_roll, false);
        assert_eq!(game.sparing, false);
        assert_eq!(game.striking_1, 0);
        assert_eq!(game.striking_2, 0);
    }

    #[test]
    fn incorrect_rolls() {
        let rolls: Vec<u8> = vec![9, 9];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.current_frame, 1);
        assert_eq!(game.remaining_rolls_in_frame, 1);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn the_wrost_game() {
        let rolls: Vec<u8> = vec![0; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 0);
        assert_eq!(game_closed(&game), true);
    }

    #[test]
    fn the_wrost_game_but_not_finished_yet() {
        let rolls: Vec<u8> = vec![0; 19];
        let game = play_this_game(&rolls);

        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn the_perfect_game() {
        let rolls: Vec<u8> = vec![10; 12];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 300);
        assert_eq!(game_closed(&game), true);
    }

    #[test]
    fn the_almost_perfect_game() {
        let rolls: Vec<u8> = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 270);
        assert_eq!(game_closed(&game), true);
    }

    #[test]
    fn a_normal_game() {
        let rolls: Vec<u8> = vec![1; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 20);
        assert_eq!(game_closed(&game), true);
    }

    #[test]
    fn recognize_spare() {
        let rolls: Vec<u8> = vec![5, 5];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 10);
        assert_eq!(game.sparing, true);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn considering_spare() {
        let rolls: Vec<u8> = vec![5, 5, 5, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 21);
        assert_eq!(game.sparing, false);
        assert_eq!(game.frame_score, 0);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn recognize_not_spare() {
        let rolls: Vec<u8> = vec![5, 4];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 9);
        assert_eq!(game.sparing, false);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn recognize_strike() {
        let rolls: Vec<u8> = vec![10];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 10);
        assert_eq!(game.striking_1, 2);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn considering_simple_strike() {
        let rolls: Vec<u8> = vec![10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 14);
        assert_eq!(game.striking_1, 0);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn go_to_next_frame_after_strike() {
        let rolls: Vec<u8> = vec![10];
        let game = play_this_game(&rolls);

        assert_eq!(game.current_frame, 2);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn a_spare_and_a_strike() {
        let rolls: Vec<u8> = vec![6, 4, 10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 34);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn a_strike_and_a_spare() {
        let rolls: Vec<u8> = vec![10, 6, 4, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 33);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn three_spares() {
        let rolls: Vec<u8> = vec![5; 7];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 50);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn nine_spares() {
        let rolls: Vec<u8> = vec![5; 19];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 140);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn ten_spares_without_last_roll() {
        let rolls: Vec<u8> = vec![5; 20];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 145);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn all_spares() {
        let rolls: Vec<u8> = vec![5; 21];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 150);
        assert_eq!(game_closed(&game), true);
    }

    #[test]
    fn two_strike() {
        let rolls: Vec<u8> = vec![10, 10];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 30);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn one_strike_and_a_open() {
        let rolls: Vec<u8> = vec![10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 14);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn two_strike_and_a_open() {
        let rolls: Vec<u8> = vec![10, 10, 1, 1];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 35);
        assert_eq!(game_closed(&game), false);
    }

    #[test]
    fn three_strike() {
        let rolls: Vec<u8> = vec![10; 3];
        let game = play_this_game(&rolls);

        assert_eq!(game.score, 60);
        assert_eq!(game_closed(&game), false);
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
        assert_eq!(game_closed(&game), true);
    }

    fn play_this_game(rolls: &Vec<u8>) -> Game {
        let mut game = new_game();
        for pins in rolls {
            roll(&mut game, *pins);
        }
        game
    }
}
