use std::error::Error;
use std::io::{self, Write};

use regex::Regex;

mod game;
use game::*;

use game::rules::Rules;

#[macro_use]
extern crate simple_error;

// Types
type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum GameType {
    Unknown,
    Normal,
    Mars,
    Venus,
}

#[derive(Debug, PartialEq)]
enum Command {
    Roll { pins: u8 },
    Score,
    Exit,
}

fn main() {
    println!("SCORING BOWLING");

    println!("  Choose the game type:");
    println!("    1) Normal rules");
    println!("    2) Mars rules");
    println!("    3) Venus rules");

    let game_type = ask_game_type();
    let rules = get_rules_from_game_type(&game_type);

    println!("Starting game with {:?} rules. Good luck!", game_type);

    println!("  Commands:");
    println!("    roll N - N pins rolled (0 to 10)");
    println!("    score - print score of current game");
    println!("    exit - exit from game");
    println!();

    let mut game = Game::new(rules);

    while !game.closed() {
        print!("Command: ");
        let _ = io::stdout().flush();

        let user_command = read_command();
        match translate_command(&user_command) {
            Ok(Command::Exit) => {
                println!("Bye.");
                std::process::exit(0);
            }
            Ok(Command::Score) => {
                println!("Score: {}", game.score());
            }
            Ok(Command::Roll { pins }) => {
                println!("Rolled {} pins", pins);
                if !game.roll(pins) {
                    println!("Invalid pins");
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }

    println!("Game over - final score: {}", game.score());
}

fn get_rules_from_game_type(game_type: &GameType) -> Rules {
    let mut rules = Rules::new();
    match game_type {
        GameType::Mars => {
            rules.max_frames = 12;
            rules.rolls_per_frame = 3;
        }
        GameType::Venus => {
            rules.initial_pins = 1;
            rules.pins_increment_per_frame = 1;
        }
        GameType::Normal => {}
        _ => panic!("Unknown game type"),
    }

    rules
}

fn ask_game_type() -> GameType {
    let mut game_type = GameType::Unknown;

    while game_type == GameType::Unknown {
        print!("Game type: ");
        let _ = io::stdout().flush();

        let user_command = read_command();
        game_type = translate_game_type(&user_command);
    }

    game_type
}

fn translate_game_type(user_command: &str) -> GameType {
    let game_type_re: Regex = Regex::new("^\\s*(?P<game>([1-3])?)\\s*$").unwrap();

    let game_code: u8 = match game_type_re.captures(user_command) {
        Some(c) => c["game"].trim().parse::<u8>().unwrap(),
        _ => {
            println!("Invalid game type.");
            0
        }
    };

    match game_code {
        1 => GameType::Normal,
        2 => GameType::Mars,
        3 => GameType::Venus,
        _ => GameType::Unknown,
    }
}

// Get command from console
fn read_command() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read from command line");
    line
}

// From user string to command
fn translate_command(command: &str) -> BoxResult<Command> {
    let roll_re: Regex = Regex::new("roll\\s+(?P<pins>(10|[0-9]))\\s*$").unwrap();
    let normalized_command = command.trim().to_lowercase();

    if normalized_command.starts_with("roll") {
        match roll_re.captures(&normalized_command) {
            Some(c) => {
                let p: u8 = c["pins"].trim().parse()?;
                Ok(Command::Roll { pins: p })
            }
            _ => bail!("invalid pins"),
        }
    } else {
        match normalized_command.as_str() {
            "score" => Ok(Command::Score),
            "exit" => Ok(Command::Exit),
            _ => bail!("invalid command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn unknown_command() {
        let command = translate_command("unknown");
        assert_eq!(command.unwrap_err().to_string(), "invalid command");
    }

    #[test]
    fn correct_exit() {
        let command = translate_command("exit");
        assert_eq!(command.unwrap(), Command::Exit);
    }

    #[test]
    fn correct_non_trimmed_exit() {
        let command = translate_command(" Exit  ");
        assert_eq!(command.unwrap(), Command::Exit);
    }

    #[test]
    fn correct_score() {
        let command = translate_command("score");
        assert_eq!(command.unwrap(), Command::Score);
    }

    #[test]
    fn correct_non_trimmed_score() {
        let command = translate_command("scORe ");
        assert_eq!(command.unwrap(), Command::Score);
    }

    // Test roll input

    #[test]
    fn correct_roll_n_pins() {
        for n in 0..10 {
            let command = translate_command(format!("roll {}", n).as_str());
            assert_eq!(command.unwrap(), Command::Roll { pins: n });
        }
    }

    #[test]
    fn incorrect_roll_n_pins() {
        for n in 11..100 {
            let command = translate_command(format!("roll {}", n).as_str());
            assert_eq!(command.unwrap_err().to_string(), "invalid pins");
        }
    }

    #[test]
    fn correct_non_trimmed_roll() {
        let command = translate_command(" roLL  0  ");
        assert_eq!(command.unwrap(), Command::Roll { pins: 0 });
    }

    #[test]
    fn incorrect_roll_invalid_format() {
        let command = translate_command("roll0");
        assert_eq!(command.unwrap_err().to_string(), "invalid pins");
    }

    #[test]
    fn incorrect_roll_missing_number() {
        let command = translate_command("roll ");
        assert_eq!(command.unwrap_err().to_string(), "invalid pins");
    }

    #[test]
    fn incorrect_roll_invalid_number() {
        let command = translate_command("roll ab");
        assert_eq!(command.unwrap_err().to_string(), "invalid pins");
    }

    // Test game type input

    #[test]
    fn correct_normal_type() {
        let command = translate_game_type("1");
        assert_eq!(command, GameType::Normal);
    }

    #[test]
    fn correct_mars_type() {
        let command = translate_game_type("2");
        assert_eq!(command, GameType::Mars);
    }

    #[test]
    fn correct_venus_type() {
        let command = translate_game_type("3");
        assert_eq!(command, GameType::Venus);
    }

    #[test]
    fn incorrect_game_type() {
        let command = translate_game_type("0");
        assert_eq!(command, GameType::Unknown);
    }

    #[test]
    fn invalid_game_type() {
        let command = translate_game_type("aa");
        assert_eq!(command, GameType::Unknown);
    }
}
