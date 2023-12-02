use std::fs;

#[derive(Clone, Copy)]
struct Window {
    values: (i32, i32, i32),
}

impl Window {
    fn sum(self) -> i32 {
        self.values.0 + self.values.1 + self.values.2
    }

    fn is_valid(self) -> bool {
        self.values.0 > 0 && self.values.1 > 0 && self.values.2 > 0
    }

    fn update(&mut self, new_value: i32) {
        self.values = (self.values.1, self.values.2, new_value);
    }
}

fn main() {
    let filename = "./input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.split("\n");
    let values: Vec<i32> = lines.map(|s: &str| -> i32 { s.parse().unwrap() }).collect();

    let mut count = 0;
    let mut prev = 0;
    let mut window = Window { values: (0, 0, 0) };

    for value in values {
        let is_valid = window.is_valid();
        window.update(value);
        let curr = window.sum();

        if curr > prev && is_valid {
            count = count + 1;
        }
        prev = curr;
    }

    println!("Found count: {}", count);
}
