use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let card_str = s.split(":").nth(1).unwrap();
        let mut card_split = card_str.split("|");

        let winning_numbers = card_split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let numbers = card_split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        Card {
            winning_numbers,
            numbers,
        }
    }
}

impl Card {
    fn count_winners(&self) -> usize {
        let mut count = 0;

        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }

        count
    }

    fn power_score(&self) -> usize {
        let count = self.count_winners();

        if count == 0 {
            return 0;
        }

        2_usize.pow(count as u32 - 1)
    }
}

fn main() {
    let filename = "./input.txt";

    let content = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = content.lines();

    // Part 1
    let sum: usize = lines
        .clone()
        .map(|line| Card::from(line).power_score())
        .sum();
    println!("Part 1: {:?}", sum);

    // Part 2
    let mut index = 1;
    let mut map: HashMap<usize, (Card, usize)> = HashMap::new();
    let mut total = 0;

    for line in lines {
        let card = Card::from(line);

        map.insert(index, (card, 1));
        index += 1;
    }

    // For each card, get the winner_count, and add count to each of the winner_count following Cards' count
    for index in 1..index {
        let (card, curr_count) = map.get(&index).unwrap().clone();
        let winner_count = card.count_winners();

        for i in 1..winner_count + 1 {
            let entry = map.entry(index + i);
            entry.and_modify(|(_, count)| *count += curr_count);
        }

        total += curr_count;
    }

    println!("Part 2: {:?}", total);
}
