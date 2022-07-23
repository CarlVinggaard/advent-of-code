use std::fmt;
use std::result::Result;

pub struct DotProductError;
pub struct MatrixMultiplicationError;

pub struct Matrix {
    rows: usize,
    cols: usize,
    pub content: Vec<Vec<isize>>,
}

#[test]
fn dot_product_test() {
    let vec1 = Vec::from([2, -3]);
    let vec2 = Vec::from([-4, 2]);

    let result1 = dot_product(&vec1, &vec2);

    if let Ok(product) = result1 {
        assert_eq!(product, -14);
    }

    let vec3 = Vec::from([1, 2, 3]);
    let vec4 = Vec::from([4, 5]);

    let result2 = dot_product(&vec3, &vec4);

    assert!(result2.is_err());
}

#[test]
fn mul_test() {
    let row1: Vec<isize> = Vec::from([1, 2, 3]);
    let row2: Vec<isize> = Vec::from([4, 5, 6]);

    let matrix = Matrix::new(Vec::from([row1, row2]));

    let multiplied_matrix = matrix.mul(2);

    assert_eq!(multiplied_matrix.content[0][0], 2);
    assert_eq!(multiplied_matrix.content[0][1], 4);
    assert_eq!(multiplied_matrix.content[1][0], 8);
    assert_eq!(multiplied_matrix.content[1][2], 12);
}

#[test]
fn transpose_test() {
    let row1: Vec<isize> = Vec::from([1, 0, 1]);
    let row2: Vec<isize> = Vec::from([2, 1, 1]);
    let row3: Vec<isize> = Vec::from([0, 1, 1]);

    let matrix = Matrix::new(Vec::from([row1, row2, row3]));

    let transposed = matrix.transpose();

    assert_eq!(transposed.content[0][0], 1);
    assert_eq!(transposed.content[1][0], 0);
    assert_eq!(transposed.content[0][1], 2);
    assert_eq!(transposed.content[2][0], 1);
}

#[test]
fn matmul_test() {
    let row1_1: Vec<isize> = Vec::from([1, 0, 1]);
    let row1_2: Vec<isize> = Vec::from([2, 1, 1]);
    let row1_3: Vec<isize> = Vec::from([0, 1, 1]);
    let row1_4: Vec<isize> = Vec::from([1, 1, 2]);

    let row2_1: Vec<isize> = Vec::from([1, 2, 1]);
    let row2_2: Vec<isize> = Vec::from([2, 3, 1]);
    let row2_3: Vec<isize> = Vec::from([4, 2, 2]);

    let matrix1 = Matrix::new(Vec::from([row1_1, row1_2, row1_3, row1_4]));
    let matrix2 = Matrix::new(Vec::from([row2_1, row2_2, row2_3]));

    let result = matrix1.matmul(&matrix2);

    if let Ok(product) = result {
        assert_eq!(product.rows, 4);
        assert_eq!(product.cols, 3);
        assert_eq!(product.content[0][0], 5);
        assert_eq!(product.content[0][1], 4);
        assert_eq!(product.content[0][2], 3);
        assert_eq!(product.content[1][0], 8);
        assert_eq!(product.content[1][1], 9);
        assert_eq!(product.content[2][1], 5);
        assert_eq!(product.content[3][2], 6);
    } else {
        panic!();
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        output.push_str("\n");

        for row in &self.content {
            output.push_str("| ");

            for el in row {
                output.push_str(el.to_string().as_str());
                output.push_str(" ");
            }

            output.push_str("|\n");
        }

        write!(f, "{}", output)
    }
}

fn dot_product(a: &Vec<isize>, b: &Vec<isize>) -> Result<isize, DotProductError> {
    if a.len() != b.len() {
        return Err(DotProductError);
    }

    let mut product = 0;

    for i in 0..a.len() {
        product += a[i] * b[i];
    }

    Ok(product)
}

impl Matrix {
    pub fn new(content: Vec<Vec<isize>>) -> Self {
        let rows = content.len();
        let cols = content[0].len();

        Matrix {
            rows,
            cols,
            content,
        }
    }

    pub fn transpose(&self) -> Self {
        let content: Vec<Vec<isize>> = (0..self.content[0].len())
            .map(|i| {
                self.content
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<isize>>()
            })
            .collect();

        Matrix::new(content)
    }

    pub fn mul(&self, factor: isize) -> Self {
        let content = self
            .content
            .iter()
            .map(|row| row.iter().map(|el| el * factor).collect())
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            content,
        }
    }

    pub fn matmul(&self, matrix: &Matrix) -> Result<Matrix, MatrixMultiplicationError> {
        if self.cols != matrix.rows {
            return Err(MatrixMultiplicationError);
        }

        let rows = self.rows;
        let cols = matrix.cols;

        let mut content = Vec::new();

        let transposed = matrix.transpose();

        for i in 0..rows {
            let mut row = Vec::<isize>::new();

            for j in 0..cols {
                let result = dot_product(&self.content[i], &transposed.content[j]);

                match result {
                    Ok(product) => row.push(product),
                    Err(_) => panic!(),
                }
            }

            content.push(row);
        }

        let product = Matrix {
            rows: self.rows,
            cols: matrix.cols,
            content,
        };

        Ok(product)
    }
}
