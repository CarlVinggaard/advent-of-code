#[derive(Debug, Clone)]
struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Number {
    x1: usize,
    x2: usize,
    y: usize,
    value: usize,
}

#[derive(Debug, Clone)]
struct Schematic {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl From<&str> for Schematic {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let mut row = 0;
        let mut symbols = Vec::new();
        let mut numbers = Vec::new();

        for line in s.lines() {
            let mut current_number = None;
            row += 1;

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
                    '0'..='9' => {
                        let digit = c.to_digit(10).unwrap();
                        if let Some((number, x1)) = current_number {
                            current_number = Some((number * 10 + digit as usize, x1));
                        } else {
                            current_number = Some((digit as usize, x));
                        }
                    }
                    _ => {
                        symbols.push(Symbol {
                            symbol: c,
                            x,
                            y: row,
                        });
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
                }
            }

            if let Some(number) = current_number {
                numbers.push(Number {
                    x1: number.1,
                    x2: width - 1,
                    y: row,
                    value: number.0,
                });
            }
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

    fn get_numbers_in_window(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<&Number> {
        self.numbers
            .iter()
            .filter(|n| n.x1 <= x2 && n.x2 >= x1 && n.y >= y1 && n.y <= y2)
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

    fn sum_gear_products(&self) -> usize {
        let mut sum = 0;

        for symbol in self.symbols.iter() {
            if symbol.symbol != '*' {
                continue;
            }

            let adjacent_numbers = self.get_numbers_in_window(
                std::cmp::max(1, symbol.x) - 1,
                std::cmp::max(1, symbol.y) - 1,
                symbol.x + 1,
                symbol.y + 1,
            );

            if adjacent_numbers.len() >= 2 {
                let mut product = 1;

                for number in adjacent_numbers {
                    product *= number.value;
                }

                sum += product;
            }
        }

        sum
    }
}

fn main() {
    let filename = "./input.txt";

    let input = std::fs::read_to_string(filename).unwrap();

    let schematic = Schematic::from(input.as_str());

    println!("Sum: {:?}", schematic.sum_numbers_adjacent_to_symbol());
    println!("Product: {:?}", schematic.sum_gear_products());
}
