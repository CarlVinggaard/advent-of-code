use std::fs;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_u32(str: &str) -> u32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!(),
    }
}

fn main() {
    let filename = "./input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let mut sum = 0;

    for line in lines {
        let mut first_digit_idx = line.find(|x: char| x.is_digit(10));
        let mut last_digit_idx = line.rfind(|x: char| x.is_digit(10));

        let mut first_str = None;
        let mut last_str = None;

        for number_str in NUMBERS {
            if let Some(i) = line.find(number_str) {
                if let Some(j) = first_digit_idx {
                    if i < j {
                        first_digit_idx = Some(i);
                        first_str = Some(number_str);
                    }
                } else {
                    first_digit_idx = Some(i);
                    first_str = Some(number_str);
                }
                if let Some(j) = last_digit_idx {
                    if i > j {
                        last_digit_idx = Some(i);
                        last_str = Some(number_str);
                    }
                } else {
                    last_digit_idx = Some(i);
                    last_str = Some(number_str);
                }
            }
        }

        let first = if let Some(str) = first_str {
            to_u32(str)
        } else {
            line.chars()
                .nth(first_digit_idx.unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap()
        };

        let last = if let Some(str) = last_str {
            to_u32(str)
        } else {
            line.chars()
                .nth(last_digit_idx.unwrap())
                .unwrap()
                .to_digit(10)
                .unwrap()
        };

        sum += first * 10 + last;
    }

    println!("{sum}");
}
