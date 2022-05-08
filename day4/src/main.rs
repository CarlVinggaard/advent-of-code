use std::fs;
use std::fmt;

fn row_is_complete(row: &[u32; 5], drawn: &Vec<u32>) -> bool {
    for number in row {
        if !drawn.contains(number) {
            return false
        }
    }

    true
}

fn parse_board(b: &str) -> Board {
    let mut rows = [[0; 5]; 5];

    let board_vec: Vec<[u32; 5]> = b.split("\n").map(|row| -> [u32; 5] {
        let mut parsed = [0; 5];

        let row_vec: Vec<u32> = 
            row.split(" ")
            .map(|c| { c.trim() })
            .filter(|c| { *c != "" } )
            .map(|n| { n.parse::<u32>().unwrap() })
            .collect();

        for i in 0..5 {
            parsed[i] = row_vec[i];
        }

        parsed
    }).collect();

    for i in 0..5 {
        rows[i] = board_vec[i]
    }

    Board { rows, won: false }
}

struct Board {
    rows: [[u32; 5]; 5],
    won: bool
}

impl<'a> std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {}\n{} {} {} {} {}\n{} {} {} {} {}\n{} {} {} {} {}\n{} {} {} {} {}\n",
            self.rows[0][0], self.rows[0][1], self.rows[0][2], self.rows[0][3], self.rows[0][4],
            self.rows[1][0], self.rows[1][1], self.rows[1][2], self.rows[1][3], self.rows[1][4],
            self.rows[2][0], self.rows[2][1], self.rows[2][2], self.rows[2][3], self.rows[2][4],
            self.rows[3][0], self.rows[3][1], self.rows[3][2], self.rows[3][3], self.rows[3][4],
            self.rows[4][0], self.rows[4][1], self.rows[4][2], self.rows[4][3], self.rows[4][4],
        )
    }
}

impl Board {
    fn get_cols(&self) -> [[u32; 5]; 5] {
        let mut cols = [[0; 5]; 5];

        for i in 0..5 {
            let col = self.rows.map(|row| -> u32 { row[i] });
            cols[i] = col;
        }

        cols
    }

    fn has_won(&self, drawn: &Vec<u32>) -> bool {
        let cols = self.get_cols();

        for row in &self.rows {
            if row_is_complete(row, &drawn) {
                return true
            }
        }

        for col in &cols {
            if row_is_complete(col, &drawn) {
                return true
            }
        }

        return false
    }

    fn get_remaining_numbers(&self, drawn: &Vec<u32>) -> Vec<u32> {
        let mut remaining: Vec<u32> = vec!();

        for row in &self.rows {
            for number in row {
                if !drawn.contains(number) {
                    remaining.push(*number);
                }
            }
        }

        remaining
    }
}

fn main() {
    let numbers_filename = "./input.txt";
    let boards_filename = "./boards.txt";

    let numbers_input = fs::read_to_string(numbers_filename).expect("There was a problem reading the file");
    let boards_input = fs::read_to_string(boards_filename).expect("There was a problem reading the file");

    let numbers: Vec<u32> = numbers_input.split(",").map(|i| { i.parse::<u32>().unwrap() }).collect();
    let mut boards: Vec<Board> = boards_input.split("\n\n").map(parse_board).collect();

    let mut drawn: Vec<u32> = vec!();

    for number in numbers {
        drawn.push(number);

        for board in &mut boards {
            if board.won { continue }

            if board.has_won(&drawn) {
                println!("A board has won! The winning board is\n{}", board);

                board.won = true;

                let remaining = board.get_remaining_numbers(&drawn);

                let sum = remaining.iter().fold(0, |a, b| { a + b });

                let product = sum * number;

                println!("A board has won! Product of sum ({}) and drawn number ({}) is {}", sum, number, product);
            }
        }
    }
}
