pub struct Matrix {
    rows: usize,
    columns: usize,
    pub mat: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mat = vec![vec![0.0; columns]; rows];
        Self { rows, columns, mat }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.columns != other.rows {
            panic!("Matrice dimensions do not match, can't multiply.");
        }
        let new = Matrix::new(self.rows, other.columns);

        new
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}
