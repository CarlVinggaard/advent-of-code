use std::fs;
use std::string::String;
use std::char;

fn get_reverse_binary_digit(d: u32, total: u32) -> u32 {
    if d < total / 2 {
        1
    } else {
        0
    }
}

fn get_binary_digit(d: u32, total: u32) -> u32 {
    if d >= total / 2 {
        1
    } else {
        0
    }
}

fn binary_to_string(bin: [u32; 12]) -> String {
    let mut out = String::new();

    for c in bin {
        out.push(char::from_digit(c, 10).unwrap());
    }

    out
}

#[derive(Debug, Copy, Clone )]
struct Counter {
    counts: [u32; 12],
    total: u32
}

impl<'a> Counter {
    fn update(&mut self, line: [u32; 12]) {

        for (i, c) in line.into_iter().enumerate() {
            self.counts[i] = self.counts[i] + c;
        }

        self.total = self.total + 1;
    }

    fn get_gamma(self) -> [u32; 12] {
        self.counts.map(|c| { get_binary_digit(c, self.total) })
    }

    fn get_epsilon(self) -> [u32; 12] {
        self.counts.map(|c| { get_reverse_binary_digit(c, self.total)})
    }
}

fn main() {
    let filename = "./input.txt";

    let contents = fs::read_to_string(filename).expect("There was a problem reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut counter = Counter { counts: [0; 12], total: 0 };

    for line in &lines {
        let chars: Vec<u32> = line.chars().map(|c| { c.to_digit(10).unwrap() }).collect();
        let binary: [u32; 12] = [chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6], chars[7], chars[8], chars[9], chars[10], chars[11]];

        counter.update(binary);
    }

    let gamma_bin = counter.get_gamma();
    let epsilon_bin = counter.get_epsilon();

    let gamma_str = binary_to_string(gamma_bin);
    let epsilon_str = binary_to_string(epsilon_bin);

    
    let gamma = u32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon_str.as_str(), 2).unwrap();
    
    println!("Power consumption: {}", gamma * epsilon);
    
    let mut oxygen_filter = lines.clone();
    let mut co2_filter = lines;
    
    // Reduce input by similarity to most common bits
    for i in 0..12 {
        if oxygen_filter.len() == 1 {
            continue
        }

        let mut counter = Counter { counts: [0; 12], total: 0 };

        for line in &oxygen_filter {
            let chars: Vec<u32> = line.chars().map(|c| { c.to_digit(10).unwrap() }).collect();
            let binary: [u32; 12] = [chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6], chars[7], chars[8], chars[9], chars[10], chars[11]];

            counter.update(binary);
        }

        let most_common_bit = get_binary_digit(counter.counts[i], counter.total);

        oxygen_filter = oxygen_filter.into_iter().filter(|line| {
            let digits: Vec<u32> = line.chars().map(|c| { c.to_digit(10).unwrap() }).collect();
            most_common_bit == digits[i]
        }).collect();
    }

    // Reduce input by similarity to least common bits
    for i in 0..12 {
        if co2_filter.len() == 1 {
            continue
        }

        let mut counter = Counter { counts: [0; 12], total: 0 };

        for line in &co2_filter {
            let chars: Vec<u32> = line.chars().map(|c| { c.to_digit(10).unwrap() }).collect();
            let binary: [u32; 12] = [chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6], chars[7], chars[8], chars[9], chars[10], chars[11]];

            counter.update(binary);
        }

        println!("Counts (index: {}, total: {}): {}", i, counter.total, counter.counts[i]);

        let least_common_bit = get_reverse_binary_digit(counter.counts[i], counter.total);

        println!("Least common bit in index {}: {}", i, least_common_bit);

        co2_filter = co2_filter.into_iter().filter(|line| {
            let digits: Vec<u32> = line.chars().map(|c| { c.to_digit(10).unwrap() }).collect();
            least_common_bit == digits[i]
        }).collect();

        println!("CO2: After index {}, {} lines remain", i, co2_filter.len())
    }

    println!("oxygen filter: {}", oxygen_filter[0]);
    println!("co2 filter: {}", co2_filter[0]);

    let oxygen_rating = u32::from_str_radix(oxygen_filter[0], 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_filter[0], 2).unwrap();

    println!("oxygen rating: {}", oxygen_rating);
    println!("CO2 rating: {}", co2_rating);

    println!("Life support rating: {}", oxygen_rating * co2_rating);
}
