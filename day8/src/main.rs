use std::fs;
use std::fmt;

fn every_contains(codes: &Vec<&str>, c: char) -> bool {
    for code in codes {
        if !code.contains(c) {
            return false
        }
    }

    true
}

struct Configuration<'a> {
    input: &'a str,
    output: &'a str,
    a: Option<char>,
    b: Option<char>,
    c: Option<char>,
    d: Option<char>,
    e: Option<char>,
    f: Option<char>,
    g: Option<char>
}

impl<'a> Configuration<'a> {
    fn new(input: &'a str, output: &'a str) -> Self {
        Configuration { input, output, a: None, b: None, c: None, d: None, e: None, f: None, g: None }
    }

    fn solve(&mut self) {
        let input: Vec<&str> = self.input.split(" ").collect();

        let mut one: &str = "";
        let mut four: &str = "";
        let mut five: &str = "";
        let mut seven: &str = "";
        let mut eight: &str = "";
        let mut chars_5: Vec<&str> = vec!();
        let mut chars_6: Vec<&str> = vec!();

        // Sort codes
        for code in input {
            match code.len() {
                2 => one = code,
                3 => seven = code,
                4 => four = code,
                7 => eight = code,
                5 => { chars_5.push(code); },
                6 => { chars_6.push(code); },
                _ => println!("Weird code found: {}", code),
            }
        }

        // Identify a
        for c in String::from(seven).chars() {
            if !one.contains(c) {
                self.a = Some(c);
            }
        }

        // Identify d
        for c in String::from(four).chars() {
            if every_contains(&chars_5, c) {
                self.d = Some(c);
                break
            }
        }

        // Identify b
        for c in String::from(four).chars() {
            if !one.contains(c) && self.d != Some(c) {
                self.b = Some(c)
            }
        }

        // Identify five
        for code in &chars_5 {
            if code.contains(self.b.unwrap()) {
                five = code;
                break
            }
        }

        // Identify c and f
        for c in String::from(one).chars() {
            if !five.contains(c) {
                self.c = Some(c);
            } else {
                self.f = Some(c);
            }
        }

        // Identify g
        for c in String::from(five).chars() {
            if every_contains(&chars_5, c) && Some(c) != self.a && Some(c) != self.d {
                self.g = Some(c);
            }
        }

        // Identify e
        for c in String::from(eight).chars() {
            if ![self.a, self.b, self.c, self.d, self.f, self.g].contains(&Some(c)) {
                self.e = Some(c)
            }
        }
    }

    fn interpret(&self) -> Result<u32, &str> {
        if [self.b, self.e, self.c].contains(&None) {
            return Err("Configuration not solved")
        }

        let mut result = [0; 4];

        for (i, code) in self.output.split(" ").enumerate() {
            match code.len() {
                2 => { result[i] = 1; continue },
                3 => { result[i] = 7; continue },
                4 => { result[i] = 4; continue },
                7 => { result[i] = 8; continue },
                6 => {
                    if !code.contains(self.e.unwrap()) {
                        result[i] = 9;
                    } else if !code.contains(self.c.unwrap()) {
                        result[i] = 6
                    } else {
                        result[i] = 0
                    }
                    continue
                },
                5 => {
                    if code.contains(self.b.unwrap()) {
                        result[i] = 5
                    } else if code.contains(self.e.unwrap()) {
                        result[i] = 2
                    } else {
                        result[i] = 3
                    }
                    continue
                },
                _ => println!("Weird code encountered!")
            }
        }

        let result_int = result[0] * 1000 + result[1] * 100 + result[2] * 10 + result[3];

        Ok(result_int)
    }
}

impl<'a> fmt::Display for Configuration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}",
            self.a.unwrap(), self.b.unwrap(), self.c.unwrap(), self.d.unwrap(), self.e.unwrap(), self.f.unwrap(), self.g.unwrap()
        )
    } 
}

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");
    
    let codes: Vec<(&str, &str)> = input.split("\n").map(|l| {
        let vec: Vec<&str> = l.split(" | ").collect();
        (vec[0].clone(), vec[1].clone())
    } ).collect(); 

    let mut count = 0u32;

    let mut total = 0;

    for pair in codes {
        let output: Vec<&str> = pair.1.split(" ").collect();

        for code in output {
            if ([2, 3, 4, 7] as [usize; 4]).contains(&code.len()) {
                count += 1;
            }
        }

        let mut config = Configuration::new(pair.0, pair.1);
        config.solve();

        let res = config.interpret();

        if let Ok(code) = res {
            total += code;
            println!("Found code: {}", code);
        }
    }

    println!("Occurences of 1,4,7 or 8: {}", count);
    println!("Total: {}", total);
}
