use std::fs;

// Two coordinates of the point followed by a reference to its value
type Point<'a> = (usize, usize, &'a u32);

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let lines: Vec<Vec<u32>> = input
        .split("\n")
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    let mut horizontal_lows: Vec<Point> = vec![];

    // Find potential lows by looking only at the horizontal direction
    for (i, line) in lines.iter().enumerate() {
        for (j, number) in line.iter().enumerate() {
            let prev = if j > 0 { &line[j - 1] } else { &10 };
            let next = if j < line.len() - 1 {
                &line[j + 1]
            } else {
                &10
            };
            if next > number && prev > number {
                horizontal_lows.push((i, j, number))
            }
        }
    }

    let mut lows: Vec<&Point> = vec![];

    // Check all potential low points in the vertical direction
    for point in horizontal_lows.iter() {
        let above = if point.0 > 0 {
            lines[point.0 - 1][point.1]
        } else {
            10
        };
        let below = if point.0 < lines.len() - 1 {
            lines[point.0 + 1][point.1]
        } else {
            10
        };

        if &above > point.2 && &below > point.2 {
            lows.push(point);
        }
    }

    // Sum up all the low points with 1 added
    let sum = lows.iter().fold(0, |sum, p| sum + p.2 + 1);
    println!("Sum of all low points (with 1 added to each): {}", sum);

    // Part 2
    // -- Depth first search from each low point to identify basins
    let mut basin_sizes: Vec<usize> = vec![];

    let limit_x = (lines.len() - 1) as i32;
    let limit_y = (lines[0].len() - 1) as i32;

    for low in lows {
        let mut basin: Vec<(usize, usize)> = vec![];
        basin.push((low.0 as usize, low.1 as usize));

        // Initialize stack
        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((low.0 as usize, low.1 as usize));

        while stack.len() > 0 {
            let point = stack.pop().unwrap();

            for rel in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let adj = (point.0 as i32 + rel.0, point.1 as i32 + rel.1);

                // Check if point is out of bounds
                if adj.0 < 0 || adj.1 < 0 || adj.0 > limit_x || adj.1 > limit_y {
                    continue;
                }

                let x = adj.0 as usize;
                let y = adj.1 as usize;

                // Check if point has already been considered
                if basin.contains(&(x, y)) {
                    continue;
                }

                // Check if point is a 9
                if lines[x][y] == 9 {
                    continue;
                }

                // Else add the point to the basin and the stack
                basin.push((x, y));
                stack.push((x, y));
            }
        }

        basin_sizes.push(basin.len());
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));

    println!(
        "Product of three largest basins: {}",
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    );
}
