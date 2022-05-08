use std::fs;
use std::cmp;

fn factorial(n: i32) -> i32 {
    let mut total = 0;

    for i in 0..n + 1 {
        total += i;
    }

    total
}

fn calculate_cost(positions: &Vec<u32>, pos: u32) -> u32 {
    let mut total_cost = 0u32;

    for _pos in positions {
        let dist: i32 = (*_pos as i32 - pos as i32).abs();
        let cost = factorial(dist);
        total_cost += cost as u32;
    }

    total_cost
}

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");
    
    let positions: Vec<u32> = input.split(",").map(|n| { n.parse::<u32>().unwrap() } ).collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut min_cost: u32 = calculate_cost(&positions, min);
    let mut best_pos: u32 = min;

    for pos in min..max {
        let cost = calculate_cost(&positions, pos);

        if cost < min_cost {
            best_pos = pos;
            min_cost = cost;
        }
        min_cost = cmp::min(cost, min_cost);
    }

    println!("minimum cost position: {} at cost: {}", best_pos, min_cost);
}
