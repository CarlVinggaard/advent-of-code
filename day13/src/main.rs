use std::fs;

struct Paper {
    x_length: usize,
    y_length: usize,
    dots: Vec<(usize, usize)>,
}

#[derive(PartialEq)]
enum Direction {
    X,
    Y,
}

impl Paper {
    fn fold(&mut self, direction: &Direction, axis: &usize) {
        let mut dots: Vec<(usize, usize)> = self
            .dots
            .clone()
            .into_iter()
            .filter(|d| match &direction {
                Direction::X => &d.0 < axis,
                Direction::Y => &d.1 < axis,
            })
            .collect();

        let mut new_dots: Vec<(usize, usize)> = vec![];

        for dot in self.dots.iter() {
            let dim = match &direction {
                Direction::X => dot.0,
                Direction::Y => dot.1,
            };

            if &dim > axis {
                let new_dim = *axis as isize - ((dim - axis) as isize).abs();
                match &direction {
                    Direction::X => new_dots.push((new_dim as usize, dot.1)),
                    Direction::Y => new_dots.push((dot.0, new_dim as usize)),
                }
            }
        }

        for dot in new_dots {
            if !dots.contains(&dot) {
                dots.push(dot);
            }
        }

        self.dots = dots;
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("There was an error reading the file");

    let instruction_input =
        fs::read_to_string("./instructions.txt").expect("There was an error reading the file");

    let dots: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let (x_str, y_str) = l.split_once(",").unwrap();
            (
                x_str.parse::<usize>().unwrap(),
                y_str.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let instructions: Vec<(Direction, usize)> = instruction_input
        .lines()
        .map(|l| {
            let words: Vec<&str> = l.split_whitespace().collect();
            let (dir_str, axis_str) = words[2].split_once("=").unwrap();

            let dir = if dir_str == "x" {
                Direction::X
            } else {
                Direction::Y
            };
            let axis = axis_str.parse::<usize>().unwrap();

            (dir, axis)
        })
        .collect();

    let mut paper = Paper {
        dots,
        x_length: 10,
        y_length: 14,
    };

    for (dir, axis) in instructions {
        paper.fold(&dir, &axis);
    }

    let mut string = String::new();

    for i in 0..50 {
        for j in 0..50 {
            // j and i can be switched here to switch the orientation of the "display"
            if paper.dots.contains(&(j, i)) {
                string.push('#');
            } else {
                string.push('.');
            }
        }
        string.push('\n');
    }

    // Turns out it spells "EBLUBRFH" :)
    println!("Output after all folds: \n{}", string);
}
