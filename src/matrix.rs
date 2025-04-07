pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>> //Vector of Vectors (rows) each containing one of i32 (data type)
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0i32; cols]; rows];
        Self { rows, cols, data }
    }
    pub fn get(&self, row: usize, col: usize) -> Option<&i32> {
        self.data.get(row)?.get(col) //use row to access vector and col to access value inside vector
    }
    pub fn set(&mut self, row: usize, col: usize, value: i32) {
        if let Some(row_vec) = self.data.get_mut(row) {
            if let Some(elem) = row_vec.get_mut(col) {
                *elem = value;
            }
        }
    }
    pub fn display(&self) {
        for row in 0..(self.rows ) {
            for col in 0..(self.cols) {
                print!("{}", self.get(row, col).unwrap());
            }
        }
    }
}

pub fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut out = Matrix::new(a.rows, b.cols);

    // iterate through a's rows
    for i in 0..a.rows {
        //iterate through b's columns
        for j in 0..b.cols {
            //iterate through the rows of b

            for k in 0..b.rows {
                out.set(i, j,
                    out.get(i, j).unwrap() + a.get(i, k).unwrap().clone() * b.get(k, j).unwrap().clone()
                );
            }
        }
    }




    out
}







