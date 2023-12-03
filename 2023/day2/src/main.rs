use std::error::Error;
use std::fmt::Display;
use std::fs;

const LIMIT_RED: u32 = 12;
const LIMIT_BLUE: u32 = 14;
const LIMIT_GREEN: u32 = 13;

struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct ParseError {
    msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.msg.as_str())
    }
}

impl Error for ParseError {}

impl TryFrom<&str> for Draw {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(",");

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for part in parts {
            let mut split = part.trim().split(" ");

            let count = split
                .next()
                .ok_or(ParseError {
                    msg: "Error parsing draw count".to_string(),
                })?
                .parse::<u32>()
                .map_err(|e| ParseError { msg: e.to_string() })?;
            let color = split.next().ok_or(ParseError {
                msg: "Unable to parse draw color".to_string(),
            })?;

            match color {
                "red" => red = count,
                "blue" => blue = count,
                "green" => green = count,
                _ => panic!(),
            }
        }

        Ok(Draw { red, blue, green })
    }
}

impl Draw {
    fn is_valid(&self) -> bool {
        self.red <= LIMIT_RED && self.blue <= LIMIT_BLUE && self.green <= LIMIT_GREEN
    }
}

struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl TryFrom<&str> for Game {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut split = s.split(":");

        let id = split
            .next()
            .ok_or(ParseError {
                msg: "Unable to parse Game ID".to_string(),
            })?
            .split(" ")
            .last()
            .ok_or(ParseError {
                msg: "Unable to parse draw".to_string(),
            })?
            .parse::<u32>()
            .map_err(|e| ParseError { msg: e.to_string() })?;

        let draws = split
            .next()
            .ok_or(ParseError {
                msg: "Error parsing draw".to_string(),
            })?
            .split(";")
            .map(|s| Draw::try_from(s).unwrap())
            .collect();

        Ok(Game { id, draws })
    }
}

impl Game {
    fn min_counts(&self) -> (u32, u32, u32) {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for draw in self.draws.iter() {
            min_red = std::cmp::max(min_red, draw.red);
            min_blue = std::cmp::max(min_blue, draw.blue);
            min_green = std::cmp::max(min_green, draw.green);
        }

        (min_red, min_blue, min_green)
    }
}

fn main() -> Result<(), ParseError> {
    let filename = "./input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let mut sum = 0;
    let mut power = 0;

    for line in lines {
        let game = Game::try_from(line)?;

        if game.draws.iter().any(|draw| draw.is_valid()) {
            sum += game.id;
        }

        let (min_red, min_blue, min_green) = game.min_counts();

        power += min_red * min_blue * min_green;
    }

    println!("Part 1: {sum}");
    println!("Part 2: {power}");

    Ok(())
}
