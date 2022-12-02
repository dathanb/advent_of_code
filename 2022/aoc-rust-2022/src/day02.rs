use anyhow::{anyhow, Result};
use crate::shared::read_lines;

pub fn part1() -> Result<i32> {
    let lines = read_lines("data/day02.txt")?;
    let mut score = 0;
    for line in lines {
        let line = line.expect("Couldn't get one or more lines");

        let parts: Vec<&str> = line.split(" ").collect();
        let opposing_move = RockPaperScissors::parse(parts[0])?;
        let my_move = RockPaperScissors::parse(parts[1])?;

        score += my_move.get_score() + my_move.get_game_result(opposing_move).get_score();
    }

    Ok(score)
}

pub fn part2() -> Result<i32> {
    let lines = read_lines("data/day02.txt")?;
    let mut score = 0;
    for line in lines {
        let line = line.expect("Couldn't get one or more lines");

        let parts: Vec<&str> = line.split(" ").collect();
        let opposing_move = RockPaperScissors::parse(parts[0])?;
        let my_move = match parts[1] {
            "X" => opposing_move.losing_move(),
            "Y" => opposing_move.draw_move(),
            "Z" => opposing_move.winning_move(),
            _ => return Err(anyhow!("Unrecognized move spec {}", parts[1]))
        };

        score += my_move.get_score() + my_move.get_game_result(opposing_move).get_score();
    }

    Ok(score)
}

#[derive(Clone, PartialEq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

impl RockPaperScissors {
    pub fn parse(input: &str) -> Result<RockPaperScissors> {
        match input {
            "A" | "X" => Ok(RockPaperScissors::Rock),
            "B" | "Y" =>  Ok(RockPaperScissors::Paper),
            "C" | "Z" => Ok(RockPaperScissors::Scissors),
            _ => Err(anyhow!("Unrecognized symbol"))
        }
    }

    /// Return what an opponent should play to beat this play
    pub fn winning_move(&self) -> RockPaperScissors{
        match self{
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock
        }
    }

    pub fn draw_move(&self) -> RockPaperScissors {
        self.clone()
    }

    /// Return what an opponent should play to lose against this play
    pub fn losing_move(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    /// Return the result of playing this move against an opposing move
    pub fn get_game_result(&self, opposing_move: RockPaperScissors) -> GameResult {
        if opposing_move == self.winning_move() {
            return GameResult::Lose;
        } else if opposing_move == self.draw_move() {
            return GameResult::Draw;
        } else {
            return GameResult::Win;
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw
}

impl GameResult {
    pub fn get_score(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = part1()?;
        println!("{}", score);
        assert_eq!(12679, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = part2()?;
        println!("{}", score);
        assert_eq!(14470, score);
        Ok(())
    }
}