use std::fs;

struct Game {
    points: u32,
}

impl Game {
    fn new() -> Self {
        Self { points: 0 }
    }

    fn play_round(&mut self, enemy_move: &Moves, player_move: &Moves) {
        let match_result = MatchResult::calculate(enemy_move, player_move);
        self.points += match_result.points() + player_move.points();
    }
}


#[derive(Debug, PartialEq)]
enum MatchResult {
    Winner,
    Loser,
    Draw,
}

impl MatchResult {
    fn calculate(enemy_move: &Moves, player_move: &Moves) -> Self {
        match (&enemy_move, &player_move) {
            (Moves::Rock, Moves::Rock) => MatchResult::Draw,
            (Moves::Rock, Moves::Paper) => MatchResult::Winner,
            (Moves::Rock, Moves::Scissors) => MatchResult::Loser,
            (Moves::Paper, Moves::Rock) => MatchResult::Loser,
            (Moves::Paper, Moves::Paper) => MatchResult::Draw,
            (Moves::Paper, Moves::Scissors) => MatchResult::Winner,
            (Moves::Scissors, Moves::Rock) => MatchResult::Winner,
            (Moves::Scissors, Moves::Paper) => MatchResult::Loser,
            (Moves::Scissors, Moves::Scissors) => MatchResult::Draw,
        }
    }

    fn parse(result: &str) -> Self {
        match result {
            "X" => Self::Loser,
            "Y" => Self::Draw,
            "Z" => Self::Winner,
            _ => panic!("Invalid match result"),
        }
    }

    fn points(&self) -> u32 {
        match self {
            Self::Winner => 6,
            Self::Loser => 0,
            Self::Draw => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn parse(movement: &str) -> Self {
        match movement {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid movement"),
        }
    }

    fn points(&self) -> u32 {
        match self {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        }
    }

    fn generate(enemy_move: &Self, match_result: &MatchResult) -> Self {
        match (enemy_move, match_result) {
            (Moves::Rock, MatchResult::Winner) => Self::Paper,
            (Moves::Rock, MatchResult::Loser) => Self::Scissors,
            (Moves::Rock, MatchResult::Draw) => Self::Rock,
            (Moves::Paper, MatchResult::Winner) => Self::Scissors,
            (Moves::Paper, MatchResult::Loser) => Self::Rock,
            (Moves::Paper, MatchResult::Draw) => Self::Paper,
            (Moves::Scissors, MatchResult::Winner) => Self::Rock,
            (Moves::Scissors, MatchResult::Loser) => Self::Paper,
            (Moves::Scissors, MatchResult::Draw) => Self::Scissors,
        }
    }
}

pub struct Day2 {}

impl Day2 {
    pub fn run() {
        let filename = "src/day2/input.txt";
        let data = fs::read_to_string(filename).unwrap();

        // Part 1
        let mut game = Game::new();

        for line in data.split('\n') {
            let data: Vec<&str> = line.split(' ').take(2).collect();

            let enemy_move = Moves::parse(data[0]);
            let player_move = Moves::parse(data[1]);

            game.play_round(&enemy_move, &player_move);
        }

        println!("Part 1 - Total points after game: {}", game.points);

        // Part 2
        let mut game = Game::new();

        for line in data.split('\n') {
            let data: Vec<&str> = line.split(' ').take(2).collect();
            
            let enemy_move = Moves::parse(data[0]);
            let match_result = MatchResult::parse(data[1]);

            let player_move = Moves::generate(&enemy_move, &match_result);

            game.play_round(&enemy_move, &player_move);
        }

        println!("Part 2 - Total points after game: {}", game.points);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves_parse() {
        assert_eq!(Moves::Rock, Moves::parse("A"));
        assert_eq!(Moves::Paper, Moves::parse("B"));
        assert_eq!(Moves::Scissors, Moves::parse("C"));
        assert_eq!(Moves::Rock, Moves::parse("X"));
        assert_eq!(Moves::Paper, Moves::parse("Y"));
        assert_eq!(Moves::Scissors, Moves::parse("Z"));
    }

    #[test]
    fn test_moves_score() {
        assert_eq!(Moves::Rock.points(), 1);
        assert_eq!(Moves::Paper.points(), 2);
        assert_eq!(Moves::Scissors.points(), 3);
    }

    #[test]
    fn test_match_result_score() {
        assert_eq!(MatchResult::Draw.points(), 3);
        assert_eq!(MatchResult::Winner.points(), 6);
        assert_eq!(MatchResult::Loser.points(), 0);
    }

    #[test]
    fn test_match_result() {
        let result = MatchResult::calculate(&Moves::Rock, &Moves::Paper);
        assert_eq!(MatchResult::Winner, result);

        let result = MatchResult::calculate(&Moves::Paper, &Moves::Rock);
        assert_eq!(MatchResult::Loser, result);

        let result = MatchResult::calculate(&Moves::Scissors, &Moves::Scissors);
        assert_eq!(MatchResult::Draw, result);
    }

    #[test]
    fn test_match_points() {
        let mut game = Game::new();

        game.play_round(&Moves::Rock, &Moves::Paper);
        game.play_round(&Moves::Paper, &Moves::Rock);
        game.play_round(&Moves::Scissors, &Moves::Scissors);

        assert_eq!(game.points, 15);
    }
}