use fnv::FnvHashMap;

use super::*;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game {
    opponent: GameChoice,
    journal_entry: JournalEntry,
}

impl Game {
    fn get_score(&self, player_move: &GameChoice) -> usize {
        let mut game_score = 0;

        match player_move {
            GameChoice::Rock => game_score += 1,
            GameChoice::Paper => game_score += 2,
            GameChoice::Scissors => game_score += 3,
        }

        match self.get_result(&player_move) {
            GameResult::Win => game_score += 6,
            GameResult::Draw => game_score += 3,
            GameResult::Loss => game_score += 0,
        }

        game_score
    }

    fn get_result(&self, player_move: &GameChoice) -> GameResult {
        match (&self.opponent, player_move) {
            (GameChoice::Scissors, GameChoice::Rock)
            | (GameChoice::Paper, GameChoice::Scissors)
            | (GameChoice::Rock, GameChoice::Paper) => GameResult::Win,
            (GameChoice::Rock, GameChoice::Scissors)
            | (GameChoice::Scissors, GameChoice::Paper)
            | (GameChoice::Paper, GameChoice::Rock) => GameResult::Loss,
            _ => GameResult::Draw,
        }
    }

    fn get_required_move(&self, desired_result: GameResult) -> GameChoice {
        match desired_result {
            GameResult::Loss => match self.opponent {
                GameChoice::Rock => GameChoice::Scissors,
                GameChoice::Scissors => GameChoice::Paper,
                GameChoice::Paper => GameChoice::Rock
            },
            GameResult::Draw => match self.opponent {
                GameChoice::Rock => GameChoice::Rock,
                GameChoice::Scissors => GameChoice::Scissors,
                GameChoice::Paper => GameChoice::Paper
            },
            GameResult::Win => match self.opponent {
                GameChoice::Rock => GameChoice::Paper,
                GameChoice::Scissors => GameChoice::Rock,
                GameChoice::Paper => GameChoice::Scissors
            },
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut game_string = value.split_ascii_whitespace();

        let opponent = GameChoice::try_from(game_string.next().unwrap())?;
        let journal_entry = JournalEntry::try_from(game_string.next().unwrap())?;

        Ok(Self {
            opponent,
            journal_entry,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for GameChoice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(anyhow!(
                "Invalid character provided for GameChoice decoding!"
            )),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum JournalEntry {
    X,
    Y,
    Z,
}

impl TryFrom<&str> for JournalEntry {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(anyhow!(
                "Invalid character provided for JournalEntry decoding!"
            )),
        }
    }
}

pub enum GameResult {
    Win,
    Draw,
    Loss,
}

pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| Game::try_from(line).unwrap())
        .collect()
}

pub fn part1(input: &[Game]) -> usize {
    let mut total_score = 0;

    for game in input {
        let player_move = match game.journal_entry {
            JournalEntry::X => GameChoice::Rock,
            JournalEntry::Y => GameChoice::Paper,
            JournalEntry::Z => GameChoice::Scissors,
        };
        total_score += game.get_score(&player_move);

    }

    total_score
}

pub fn part1_memoized(input: &[Game]) -> usize {
    let mut game_history = FnvHashMap::default();
    let mut total_score = 0;

    for game in input {
        if let Some(score) = game_history.get(game) {
            total_score += score;
        } else {
            let player_move = match game.journal_entry {
                JournalEntry::X => GameChoice::Rock,
                JournalEntry::Y => GameChoice::Paper,
                JournalEntry::Z => GameChoice::Scissors,
            };
            let game_score = game.get_score(&player_move);
            game_history.insert(game, game_score);
            total_score += game_score;
        }

    }

    total_score
}

pub fn part2(input: &[Game]) -> usize {
    let mut total_score = 0;

    for game in input {
        let desired_result = match game.journal_entry {
            JournalEntry::X => GameResult::Loss,
            JournalEntry::Y => GameResult::Draw,
            JournalEntry::Z => GameResult::Win,
        };

        let player_move = game.get_required_move(desired_result);

        total_score += game.get_score(&player_move);
    }

    total_score
}

pub fn part2_memoized(input: &[Game]) -> usize {
    let mut game_history = FnvHashMap::default();
    let mut total_score = 0;

    for game in input {
        if let Some(score) = game_history.get(game) {
            total_score += score;
        } else {
            let desired_result = match game.journal_entry {
                JournalEntry::X => GameResult::Loss,
                JournalEntry::Y => GameResult::Draw,
                JournalEntry::Z => GameResult::Win,
            };

            let player_move = game.get_required_move(desired_result);

            let game_score = game.get_score(&player_move);
            game_history.insert(game, game_score);
            total_score += game_score;
        }
    }

    total_score
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2022/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 15);
    test!(part2, 12);
}
