use std::clone::Clone;

pub struct Matrix<T> {
    data: Box<[T]>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, initial_val: T) -> Matrix<T> {
        Matrix {
            data: vec![initial_val; rows * cols].into_boxed_slice(),
            rows,
            cols,
        }
    }
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

    pub fn get(&self, row: usize, col: usize) -> &T {
        debug_assert!(row < self.rows);
        debug_assert!(col < self.cols);
        
        &self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        debug_assert!(row < self.rows);
        debug_assert!(col < self.cols);
        
        self.data[row * self.cols + col] = value;
    }
}
