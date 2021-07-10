/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // Check that the bounds for the matrices are acceptable.
    if (mat1.len() == 0 || mat2.len() == 0) || mat1[0].len() != mat2.len() {
        panic!("unexpected bounds for matrices");
    }
    // Create new matrix to store the result
    let mut res_mat: Matrix = vec![vec![0.; mat2[0].len()]; mat1.len()];

    for mat1_row in 0..mat1.len() {
        for mat2_col in 0..mat2[0].len() {
            res_mat[mat1_row][mat2_col] = (0..mat2.len())
                .fold(0., |acc, i| acc + mat1[mat1_row][i] * mat2[mat2_col][i]);
        }
    }
    return res_mat;
}