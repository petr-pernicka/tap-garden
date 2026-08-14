use std::ops::{AddAssign, Index, IndexMut, Mul};

pub struct Matrix<T> {
    data: Box<[T]>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T: Mul<Output = T> + Copy> Matrix<T> {
    #[allow(clippy::uninit_vec)]
    pub fn mul_rows(&self, rhs: &[T]) -> Matrix<T> {
        debug_assert!(rhs.len() == self.rows);

        let mut result = Vec::with_capacity(self.data.len());
        // UNSAFE: The vec is initialized immediately after.
        unsafe {
            result.set_len(self.data.len());
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i * self.cols + j] = self[(i, j)] * rhs[i];
            }
        }

        Self {
            data: result.into_boxed_slice(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<T: AddAssign + Copy> Matrix<T> {
    pub fn row_sum(&self) -> Vec<T> {
        let mut row_sum = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                row_sum[i] += self[(i, j)];
            }
        }

        row_sum
    }
}

impl<T: Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize, initial_val: T) -> Matrix<T> {
        Self {
            data: vec![initial_val; rows * cols].into_boxed_slice(),
            rows,
            cols,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        debug_assert!(row < self.rows);
        debug_assert!(col < self.cols);

        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        debug_assert!(row < self.rows);
        debug_assert!(col < self.cols);

        &mut self.data[row * self.cols + col]
    }
}
