use matrix::Matrix;

mod matrix;

struct Scanner {
    beacons: Vec<(isize, isize, isize)>,
}

fn get_permutations() -> Vec<(isize, isize, isize)> {
    Vec::from([(0, 1, 2), (0, 2, -1), (0, -1, -2), (0, -2, 1)])
}

fn reorient(
    (dx, dy, dz): (isize, isize, isize),
) -> fn(&(isize, isize, isize)) -> (isize, isize, isize) {
    |pos: &(isize, isize, isize)| {
        let mut new = (0, 0, 0);

        *pos
    }
}

impl Scanner {
    // dx, dy and dz should be either 1 or -1
    fn reorient(&self, orientation: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        self.beacons.iter().map(reorient(orientation)).collect()
    }

    fn check_overlap(&self, scanner: &Scanner) -> usize {
        0
    }
}

fn main() {
    let content_a: Vec<Vec<isize>> =
        Vec::from([Vec::from([1, 0]), Vec::from([2, 1]), Vec::from([0, 1])]);
    let a = Matrix::new(content_a);

    let content_b: Vec<Vec<isize>> = Vec::from([Vec::from([1, 2]), Vec::from([2, 3])]);

    let b = Matrix::new(content_b);

    println!("matrix A: {}", a);

    println!("matrix B: {}", b);

    let res = a.matmul(&b);

    if let Ok(product) = res {
        println!("product: {}", product);
    }
}
