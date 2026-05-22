pub struct Matrix {
    pub data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    #[inline]
    pub fn read(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }

    #[inline]
    pub fn modify(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * self.cols + col] = value;
    }

    #[inline]
    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols{
                print!("{}", self.read(row, col));
                print!(" ");
            }
            println!();
        }
    }
}
