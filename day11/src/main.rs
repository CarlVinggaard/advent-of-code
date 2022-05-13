use std::fs;

struct Octopus {
    e_level: u32,
    flashed: bool,
}

struct Grid {
    grid: Vec<Vec<Octopus>>,
    flashes: u32,
}

impl Grid {
    fn increment_all(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.grid[x][y].e_level += 1
            }
        }
    }

    fn check_and_flash(&mut self, p: (usize, usize)) {
        let octopus = &self.grid[p.0][p.1];

        // Check
        if octopus.e_level < 10 || octopus.flashed { return }

        // Flash
        self.flashes += 1;
        self.grid[p.0][p.1].flashed = true;

        let x = p.0 as isize;
        let y = p.1 as isize;

        // Increment all adjancent and call check_and_flash for them
        let adjacent = [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1), 
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1)
        ];

        for (a, b) in adjacent {
            // Check if out of bounds
            if a < 0 || b < 0 || a > (self.grid.len() - 1) as isize || b > (self.grid[0].len() - 1) as isize {
                continue
            }

            self.grid[a as usize][b as usize].e_level += 1;
            self.check_and_flash((a as usize, b as usize));
        }
    }

    fn check_and_flash_all(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.check_and_flash((x, y));
            }
        }
    }

    fn reset(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                if self.grid[x][y].flashed {
                    self.grid[x][y].e_level = 0;
                    self.grid[x][y].flashed = false;
                }
            }
        }
    }

    fn all_flashed(&self) -> bool {
        for row in &self.grid {
            for octopus in row {
                if !octopus.flashed {
                    return false
                }
            }
        }

        true
    }
}

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let grid: Vec<Vec<Octopus>> = input.split("\n").map(|l| {
        l.chars().map(|c| {
            let e_level = c.to_digit(10).unwrap();
            Octopus { e_level, flashed: false }
        }).collect()
    }).collect();

    let mut grid = Grid { grid, flashes: 0 };

    for i in 1..1000 {
        grid.increment_all();
        grid.check_and_flash_all();

        if grid.all_flashed() {
            println!("All octopuses flashed in round: {}", i);
            break
        }

        grid.reset();

        if i % 10 == 0 {
            println!("Flashes after {} cycles: {}", i, grid.flashes);
        }
    }

    println!("Flashes after all cycles: {}", grid.flashes);
}
