#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Number {
    x1: usize,
    x2: usize,
    y: usize,
    value: usize,
}

#[derive(Debug)]
struct Schematic {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl From<&str> for Schematic {
    fn from(s: &str) -> Self {
        let mut row = 0;
        let mut symbols = Vec::new();
        let mut numbers = Vec::new();

        let mut current_number: Option<(usize, usize)> = None;

        for line in s.lines() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        if let Some((number, x1)) = current_number {
                            numbers.push(Number {
                                x1,
                                x2: x - 1,
                                y: row,
                                value: number,
                            });
                            current_number = None;
                        }
                    }
                    '#' | '$' | '*' | '+' | '@' | '=' | '%' | '/' | '-' | '&' => {
                        symbols.push(Symbol { x, y: row });
                        if let Some((number, x1)) = current_number {
                            numbers.push(Number {
                                x1,
                                x2: x - 1,
                                y: row,
                                value: number,
                            });
                            current_number = None;
                        }
                    }
                    '0'..='9' => {
                        let digit = c.to_digit(10).unwrap();
                        println!("digit: {}", digit);
                        println!("current_number: {:?}", current_number);
                        if let Some((number, x1)) = current_number {
                            current_number = Some((number * 10 + digit as usize, x1));
                        } else {
                            current_number = Some((digit as usize, x));
                        }
                    }
                    _ => panic!("Invalid character"),
                }
            }

            row += 1;
            current_number = None;
        }

        Self { symbols, numbers }
    }
}

impl Schematic {
    fn get_symbols_in_window(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<&Symbol> {
        self.symbols
            .iter()
            .filter(|s| s.x >= x1 && s.x <= x2 && s.y >= y1 && s.y <= y2)
            .collect()
    }

    fn sum_numbers_adjacent_to_symbol(&self) -> usize {
        let mut sum = 0;

        for number in self.numbers.iter() {
            let adjacent_symbols = self.get_symbols_in_window(
                std::cmp::max(1, number.x1) - 1,
                std::cmp::max(1, number.y) - 1,
                number.x2 + 1,
                number.y + 1,
            );

            if adjacent_symbols.len() > 0 {
                sum += number.value;
            }
        }

        sum
    }
}

fn main() {
    let filename = "./input.txt";

    let input = std::fs::read_to_string(filename).unwrap();

    let schematic = Schematic::from(input.as_str());

    // 528231 is too low
    println!("Sum: {:?}", schematic.sum_numbers_adjacent_to_symbol());
}
