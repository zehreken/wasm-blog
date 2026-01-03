pub struct Matrix {
    rows: usize,
    columns: usize,
    pub mat: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(mat: Vec<Vec<f32>>) -> Self {
        if mat.len() == 0 {
            panic!("Matrix has no rows");
        }
        if mat[0].len() == 0 {
            panic!("Matrix has no columns");
        }
        Self {
            rows: mat.len(),
            columns: mat[0].len(),
            mat,
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.columns != other.rows {
            panic!(
                "Incompatible dimensions, can't multiply. columns_a: {} rows_b: {}",
                self.columns, other.rows
            );
        }
        let mut mat = vec![vec![0.0; self.rows]; other.columns];
        // Cij = sum(Aik*Bkj) for k in [1, n]
        for i in 0..self.rows {
            for j in 0..other.columns {
                for k in 0..self.columns {
                    mat[i][j] += self.mat[i][k] * other.mat[k][j];
                }
            }
        }

        let new = Matrix::new(mat);
        new
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}

#[test]
fn test_matrix_multiplication() {
    let mat_a = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let mat_b = Matrix::new(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);

    let mat_c = mat_a.multiply(&mat_b);

    assert_eq!(mat_c.mat, vec![vec![58.0, 64.0], vec![139.0, 154.0]]);
}

#[test]
#[should_panic]
fn test_matrix_dimensions() {
    let mat_a = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let mat_b = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

    mat_a.multiply(&mat_b);
}
