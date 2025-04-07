//T is a generic type, meaning that we can generate a matrix of any type
pub struct Matrix<T> {
    height: usize, //Specifying width here is unnecessary, as Vec already provides a length,
    matrices: Vec<*const [T]>, //This is bad, but I don't give a shit
}

impl<T> Matrix<T> {
    pub fn new(height: usize) -> Matrix<T> {
        Matrix {
            height,
            matrices: Vec::new()
        }
    }
    pub fn append(&mut self, matrix: *const [T]) {
        self.matrices.push(matrix);
    }
}