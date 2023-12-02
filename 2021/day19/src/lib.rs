use nalgebra::DMatrix;

pub fn generate_permutation_matrices() -> Vec<DMatrix<isize>> {
    let permutation_indices = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let flips = [[1, 1, 1], [1, -1, -1], [-1, 1, -1], [-1, -1, 1]];

    let mut permutation_matrices = Vec::new();

    for flip in &flips {
        for permutation in &permutation_indices {
            let mut matrix_data = vec![0; 9]; // Initialize with 0's

            for (index, &perm_index) in permutation.iter().enumerate() {
                matrix_data[3 * index + perm_index] = flip[index]; // Assign flip value
            }

            let perm_matrix = DMatrix::from_row_slice(3, 3, &matrix_data);
            permutation_matrices.push(perm_matrix);
        }
    }

    permutation_matrices
}
