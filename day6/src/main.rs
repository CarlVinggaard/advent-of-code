use std::fs;

#[derive(Clone, Copy)]
struct Fish {
    days: u8
}

struct School {
    fishes: Vec<Fish>
}

impl Fish {
    fn new() -> Self {
        Fish { days: 8 }
    }

    fn update(&mut self) -> () {
        if self.days == 0 {
            self.days = 6;
        } else {
            self.days -= 1;
        }
    }
}

impl School {
    fn update(&mut self) {
        let copy: Vec<Fish> = self.fishes.to_vec();
        for (i, fish) in copy.iter().enumerate() {
            if fish.days == 0 {
                self.fishes.push(Fish::new())
            }
            self.fishes[i].update();
        }
    }

    fn get_counts(&self) -> [u128; 9] {
        let mut counts = [0u128; 9];

        for fish in &self.fishes {
            counts[fish.days as usize] += 1;
        }

        counts
    }

    fn get_count_after_cycles(&mut self, cycles: usize) -> u128 {
        let mut counts = self.get_counts();

        for _ in 0..cycles {
            let new = counts[0];
            counts[0] = counts[1];
            counts[1] = counts[2];
            counts[2] = counts[3];
            counts[3] = counts[4];
            counts[4] = counts[5];
            counts[5] = counts[6];
            counts[6] = counts[7] + new;
            counts[7] = counts[8];
            counts[8] = new;
        }

        counts.iter().sum()
    }
}

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file"); 

    let fishes: Vec<Fish> = input.split(",").map(|n| {
        let days = n.parse::<u8>().unwrap();
        Fish { days }
    }).collect();

    let mut school = School { fishes };

    let cycles = 256;

    let count = school.get_count_after_cycles(cycles);

    println!("Number of fish after {} days: {}", cycles, count);

}
