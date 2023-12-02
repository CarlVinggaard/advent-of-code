use anyhow::Result;
use lib::generate_permutation_matrices;
use nalgebra::DMatrix;
use regex::Regex;
use std::{collections::HashSet, fs};

mod lib;

struct Scanner {
    pub beacons: DMatrix<isize>,
}

impl Scanner {
    pub fn new(beacons: Vec<Vec<isize>>) -> Self {
        let matrix = DMatrix::from_row_slice(
            beacons.len(),
            3,
            beacons
                .into_iter()
                .flatten()
                .collect::<Vec<isize>>()
                .as_slice(),
        );

        Scanner { beacons: matrix }
    }

    pub fn get_permutations(&self) -> Vec<DMatrix<isize>> {
        let permutation_matrices = generate_permutation_matrices();

        let mut permutations = Vec::new();

        for perm_matrix in permutation_matrices {
            let permutation = &self.beacons * perm_matrix;
            permutations.push(permutation);
        }

        permutations
    }

    fn check_overlap(&self, beacons: &DMatrix<isize>) -> usize {
        let mut overlap = 0;

        for row_1 in self.beacons.row_iter() {
            for row_2 in beacons.row_iter() {
                if row_1 == row_2 {
                    overlap += 1;
                }
            }
        }

        overlap
    }

    fn intersection(&self, beacons: &DMatrix<isize>) -> HashSet<(isize, isize, isize)> {
        let mut self_set: HashSet<_> = HashSet::new();
        let mut beacon_set: HashSet<_> = HashSet::new();

        for row in self.beacons.row_iter() {
            self_set.insert((row[0], row[1], row[2]));
        }

        for row in self.beacons.row_iter() {
            for dx in -1000..=1000 {
                for dy in -1000..=1000 {
                    for dz in -1000..=1000 {
                        beacon_set.insert((row[0] + dx, row[1] + dy, row[2] + dz));
                    }
                }
            }
        }

        // Find and return the intersection of the two sets.
        self_set.intersection(&beacon_set).cloned().collect()
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./test.txt").expect("There was a problem reading the file");

    let scanner_separator = Regex::new(r"--- scanner \d ---\n")?;
    let scanners_input = scanner_separator.split(&input);

    let mut scanners = Vec::new();

    for input in scanners_input {
        let lines = input.lines();

        let mut beacons = Vec::new();

        for line in lines {
            let row: Vec<isize> = line
                .split(",")
                .map(|x| x.to_string().parse::<isize>())
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .collect();

            if row.len() == 3 {
                beacons.push(row);
            }
        }

        if beacons.len() > 0 {
            scanners.push(Scanner::new(beacons));
        }
    }

    for permutation in scanners[1].get_permutations() {
        let overlap = scanners[0].intersection(&permutation);

        println!("{:?}", overlap);
    }

    Ok(())
}
