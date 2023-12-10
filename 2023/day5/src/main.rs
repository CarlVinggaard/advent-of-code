use rangemap::RangeInclusiveMap;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;
use std::str::Lines;

static CONVERSIONS: [&str; 7] = [
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

#[derive(Debug, Clone)]
struct CustomMap {
    from: String,
    to: String,
    offset_map: RangeInclusiveMap<usize, isize>,
}

fn parse_custom_map(section_title: &str, lines: &mut Lines) -> CustomMap {
    let parts: Vec<&str> = section_title.split("-to-").collect();
    let from = parts[0].to_string();
    let to = parts[1].replace(" map:", "").to_string();
    let mut offset_map = RangeInclusiveMap::new();

    for line in lines {
        if line.trim().is_empty() {
            break;
        }

        // Parse range and add to ranges
        let numbers = line
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        offset_map.insert(
            numbers[1]..=numbers[1] + numbers[2],
            numbers[0] as isize - numbers[1] as isize,
        );
    }

    CustomMap {
        from,
        to,
        offset_map,
    }
}

fn get_lowest_location(
    initial_seeds: impl Iterator<Item = usize>,
    custom_maps: HashMap<String, CustomMap>,
) -> isize {
    let mut lowest_location = isize::MAX;

    for initial_seed in initial_seeds {
        let mut seed = initial_seed as isize;
        for (i, conversion) in CONVERSIONS.into_iter().enumerate() {
            let map = custom_maps.get(conversion).unwrap();

            // Check that order of conversions mathces the maps given
            if i > 0 && map.from != CONVERSIONS[i - 1] {
                panic!("Conversion order does not match maps given");
            }

            let offset = map.offset_map.get(&(seed as usize)).unwrap_or(&0);

            seed += offset;
        }

        lowest_location = lowest_location.min(seed);
    }

    lowest_location
}

fn main() {
    let filename = "./input.txt";
    let mut custom_maps = HashMap::<String, CustomMap>::new();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let initial_seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let initial_seeds_part_2: Vec<RangeInclusive<usize>> = initial_seeds
        .clone()
        .windows(2)
        .map(|w| RangeInclusive::new(w[0], w[0] + w[1]))
        .collect();

    while let Some(line) = lines.next() {
        if line.ends_with("map:") {
            let map = parse_custom_map(line, &mut lines);
            custom_maps.insert(map.clone().to, map);
        }
    }

    let lowest_location_part_1 =
        get_lowest_location(initial_seeds.iter().cloned(), custom_maps.clone());

    println!("Lowest location part 1: {}", lowest_location_part_1);

    let mut lowest_location_part_2 = isize::MAX;

    // Part 2 needs a different data model/algorithm :/
    /*
       for range in initial_seeds_part_2 {
           let lowest_location = get_lowest_location(range.clone(), custom_maps.clone());
           lowest_location_part_2 = lowest_location_part_2.min(lowest_location);
       }
    */

    println!("Lowest location part 2: {}", lowest_location_part_2);
}
