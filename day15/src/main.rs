use std::fs;

#[derive(Clone, Copy)]
struct Node {
    distance: u32,
    weight: u32,
    visited: bool,
    x: usize,
    y: usize,
}

fn wrap(w: u32) -> u32 {
    (w - 1) % 9 + 1
}

fn extend(weights: Vec<Vec<u32>>, factor: u32) -> Vec<Vec<u32>> {
    let mut extended: Vec<Vec<u32>> = Vec::new();

    extended.resize(weights.len(), Vec::new());

    // Extend width
    for i in 0..factor {
        for (j, row) in weights.iter().enumerate() {
            let mut incremented_row: Vec<u32> = row.iter().map(|w| wrap(w + i)).collect();

            extended[j].append(&mut incremented_row)
        }
    }

    let tile_copy = extended.clone();

    // Extend height
    for i in 1..factor {
        let mut incremented_tile: Vec<Vec<u32>> = tile_copy
            .iter()
            .map(|row| row.iter().map(|w| wrap(w + i)).collect())
            .collect();
        extended.append(&mut incremented_tile);
    }

    extended
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("There was an error reading the file");

    let initial_weights: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let weights = extend(initial_weights.clone(), 5);

    let mut nodes: Vec<Vec<Node>> = weights
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, weight)| Node {
                    distance: 9999,
                    weight: *weight,
                    x,
                    y,
                    visited: false,
                })
                .collect()
        })
        .collect();

    let dim_x = nodes.len() as isize;
    let dim_y = nodes[0].len() as isize;

    // Set distance of the initial node to 0
    nodes[0][0].distance = 0;

    let mut current = (0, 0);
    let mut unvisited: Vec<(usize, usize)> = Vec::from([(0, 0)]);

    while unvisited.len() > 0 {
        let current_point = nodes[current.0][current.1].clone();

        println!("current point: ({}, {})", current.0, current.1);

        // Calculate distance of all adjacent nodes through current node
        // and update their distance if a shorter one is found
        for (x1, y1) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let x = current_point.x as isize + x1;
            let y = current_point.y as isize + y1;

            // Ignore point if it is out of bounds
            if x < 0 || y < 0 || x > (dim_x - 1) || y > (dim_y - 1) {
                continue;
            }

            let node = &nodes[x as usize][y as usize].clone();

            // Calculate distance to adjacent node through current node
            let distance = current_point.distance + node.weight;

            if node.distance > distance {
                nodes[x as usize][y as usize].distance = distance;
            }

            if !node.visited {
                unvisited.push((x as usize, y as usize));
            }
        }

        // Set current point as visited
        nodes[current.0][current.1].visited = true;

        // Remove current from unvisited
        unvisited = unvisited
            .into_iter()
            .filter(|(x, y)| x != &current.0 || y != &current.1)
            .collect();

        unvisited.sort_by(|a, b| nodes[a.0][a.1].distance.cmp(&nodes[b.0][b.1].distance));

        if unvisited.len() > 0 {
            current = unvisited[0];
        }
    }

    println!(
        "Shortest distance to point: {}",
        nodes[(dim_x - 1) as usize][(dim_y - 1) as usize].distance
    );
}
