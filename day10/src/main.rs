use std::fs;
use std::str::Chars;

fn get_closing(c: &char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '{' => Some('}'),
        '[' => Some(']'),
        '<' => Some('>'),
        _ => None,
    }
}

fn check_is_corrupted(chars: &Chars) -> Option<char> {
    let mut stack: Vec<char> = vec![];
    for c in chars.clone() {
        if ['(', '{', '<', '['].contains(&c) {
            stack.push(c);
        }

        if [')', '}', '>', ']'].contains(&c) {
            let open_opt = stack.pop();

            if let Some(open) = open_opt {
                if let Some(closed) = get_closing(&open) {
                    if c != closed {
                        return Some(c);
                    }
                }
            }
        }
    }

    None
}

// We assume that there are no corrupted lines passed to this function
fn check_is_incomplete(chars: &Chars) -> Option<Vec<char>> {
    let mut stack: Vec<char> = vec![];
    for c in chars.clone() {
        if ['(', '{', '<', '['].contains(&c) {
            stack.push(c);
        }

        if [')', '}', '>', ']'].contains(&c) {
            stack.pop();
        }
    }

    if stack.len() == 0 {
        return None;
    }

    let mut remaining: Vec<char> = stack.iter().map(|c| get_closing(c).unwrap()).collect();

    remaining.reverse();

    Some(remaining)
}

fn calculate_score(line: Vec<char>) -> u64 {
    let mut score = 0;

    for c in line {
        match c {
            ')' => score = score * 5 + 1,
            ']' => score = score * 5 + 2,
            '}' => score = score * 5 + 3,
            '>' => score = score * 5 + 4,
            _ => (),
        }
    }

    score
}

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let lines: Vec<Chars> = input.split("\n").map(|l| l.chars()).collect();

    let mut corrupted_sum: u32 = 0;
    let mut scores: Vec<u64> = vec![];

    for line in lines {
        let opt = check_is_corrupted(&line);

        if let Some(c) = opt {
            match c {
                ')' => corrupted_sum += 3,
                ']' => corrupted_sum += 57,
                '}' => corrupted_sum += 1197,
                '>' => corrupted_sum += 25137,
                _ => (),
            }
        } else {
            let opt = check_is_incomplete(&line);

            if let Some(c) = opt {
                scores.push(calculate_score(c));
            }
        }
    }

    scores.sort();

    println!("Sum of all corrupted lines: {}", corrupted_sum);
    println!("Middle score: {}", scores[scores.len() / 2]);
}
