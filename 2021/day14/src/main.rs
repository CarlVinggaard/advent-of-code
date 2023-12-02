use std::collections::BTreeMap;
use std::fs;

fn parse_base(base: &str) -> Vec<String> {
    let mut prev_opt: Option<char> = None;
    let mut parsed: Vec<String> = vec![];

    for c in base.chars() {
        if let Some(prev) = prev_opt {
            let mut pair = String::from(prev);
            pair.push(c);

            parsed.push(pair);
        }
        prev_opt = Some(c);
    }

    parsed
}

fn extend_polymers(
    polymers: BTreeMap<String, usize>,
    rules: &Vec<Rule>,
) -> BTreeMap<String, usize> {
    // Loop through all entries
    // Get the outputs/extensions for each
    // Add them to new_polymers with same count
    // Return new_polymers

    let mut new_polymers = BTreeMap::<String, usize>::new();

    for (k, v) in polymers {
        let rule_opt = find_rule(&rules, &k);

        if let Some(rule) = rule_opt {
            let (pair1, pair2) = rule.get_extensions();

            *new_polymers.entry(pair1.clone()).or_insert(0) += v;
            *new_polymers.entry(pair2.clone()).or_insert(0) += v;
        }
    }

    new_polymers
}

fn find_rule<'a>(rules: &'a Vec<Rule>, pair: &'a String) -> Option<&'a Rule> {
    for rule in rules {
        if &rule.input == pair {
            return Some(rule);
        }
    }

    None
}

fn count_letters(polymers: &BTreeMap<String, usize>) -> Vec<(char, usize)> {
    let mut count_map = BTreeMap::<char, usize>::new();

    // Add up all letters in polymers
    for (k, v) in polymers {
        let first = k.chars().nth(0).unwrap();
        let second = k.chars().nth(1).unwrap();

        *count_map.entry(first).or_insert(0) += v;
        *count_map.entry(second).or_insert(0) += v;
    }

    // Since all letters are repeated, divide counts by 2
    // Two letters (in the ends) will have uneven counts - in that case half and round up
    let mut counts: Vec<(char, usize)> = vec![];

    for (k, v) in count_map {
        let h: f64 = v as f64 / 2.0;
        counts.push((k, h.ceil() as usize));
    }

    counts.sort_by(|a, b| b.1.cmp(&a.1));

    counts
}

struct Rule {
    input: String,
    output: char,
}

impl Rule {
    fn get_extensions(&self) -> (String, String) {
        let first = self.input.chars().nth(0).unwrap();
        let second = self.input.chars().nth(1).unwrap();

        let mut out1 = String::from(first);
        let mut out2 = String::from(self.output);

        out1.push(self.output);
        out2.push(second);

        (out1, out2)
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("There was a problem reading the file");

    let (base_str, rules_str) = input.split_once("\n\n").unwrap();

    let rules: Vec<Rule> = rules_str
        .lines()
        .map(|l| {
            let (input_str, output_str) = l.split_once(" -> ").unwrap();
            let output = output_str.chars().next().unwrap();
            Rule {
                input: String::from(input_str),
                output,
            }
        })
        .collect();

    let mut polymers = BTreeMap::<String, usize>::new();

    // Add initial pairs to hashmap
    let base = parse_base(base_str);

    for pair in base {
        *polymers.entry(pair).or_insert(0) += 1;
    }

    let cycles = 40;

    // Cycles
    for _ in 0..cycles {
        polymers = extend_polymers(polymers, &rules);
    }

    let counts = count_letters(&polymers);

    let max = counts.first().unwrap().1;
    let min = counts.last().unwrap().1;

    let diff = max - min;

    println!("The difference is: {}", diff);
}
