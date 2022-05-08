use std::fs;
use std::cmp;

fn parse_line(l: &str) -> Line {
    let points: Vec<(i16, i16)> = l.split(" -> ").map(|p| {
        let numbers: Vec<i16> = p.split(",").map(|n| { n.parse::<i16>().unwrap() }).collect();
        (numbers[0], numbers[1])
    }).collect();

    Line { p1: points[0], p2: points[1] }
}

struct CoordinateSystem {
    coordinates: [[i16; 1000]; 1000]
}

impl CoordinateSystem {
    fn count_covered_points(&self, min: i16) -> i16 {
        let mut count = 0;

        for row in self.coordinates.iter() {
            for point in row {
                if point >= &min {
                    count += 1;
                }
            }
        }

        count
    }
}

struct Line {
    p1: (i16, i16),
    p2: (i16, i16)
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.p1.0 == self.p2.0
    }

    fn is_horizontal(&self) -> bool {
        self.p1.1 == self.p2.1
    }

    fn is_diagonal(&self) -> bool {
        (self.p1.0 - self.p2.0).abs() == (self.p1.1 - self.p2.1).abs()
    }

    fn covers(&self) -> Vec<(i16, i16)> {
        let mut points: Vec<(i16, i16)> = vec!();

        if self.is_vertical() {
            let min = cmp::min(self.p1.1, self.p2.1);
            let max = cmp::max(self.p1.1, self.p2.1) + 1;

            for i in min..max {
                points.push((self.p1.0 as i16, i as i16))
            }
        }
        
        if self.is_horizontal() {
            let min = cmp::min(self.p1.0, self.p2.0);
            let max = cmp::max(self.p1.0, self.p2.0) + 1;

            for i in min..max {
                points.push((i, self.p1.1))
            }
        }
        
        if self.is_diagonal() {
            let len = (self.p1.0 - self.p2.0).abs() + 1;

            let x_increases = self.p2.0 > self.p1.0;
            let y_increases = self.p2.1 > self.p1.1;

            for i in 0..len {
                let x = if x_increases { self.p1.0 + i } else { self.p1.0 - i };
                let y = if y_increases { self.p1.1 + i } else { self.p1.1 - i };
                points.push((x, y));
            }
        }

        points
    }
}

fn main() {
     let filename = "./input.txt";

    let numbers_input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let all_lines: Vec<Line> = numbers_input.split("\n").map(parse_line).collect();
    let mut system = CoordinateSystem { coordinates: [[0; 1000]; 1000] };

    let lines: Vec<&Line> = all_lines.iter().filter(|l| { l.is_horizontal() || l.is_vertical() || l.is_diagonal() }).collect();

    for line in lines {
        let covers = line.covers();

        for point in covers {
            system.coordinates[point.0 as usize][point.1 as usize] += 1;
        }
    }

    let count = system.count_covered_points(2);

    println!("Found {} points covered min 2 times", count);
}
