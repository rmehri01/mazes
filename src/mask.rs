use std::ops;

pub struct Mask {
    rows: usize,
    cols: usize,
    bits: Vec<Vec<bool>>,
}

impl Mask {
    pub fn new(rows: usize, cols: usize) -> Self {
        Mask {
            rows,
            cols,
            bits: vec![vec![true; cols]; rows],
        }
    }

    pub fn num_rows(&self) -> usize {
        self.rows
    }
    pub fn num_cols(&self) -> usize {
        self.cols
    }
}

impl ops::Index<usize> for Mask {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl ops::IndexMut<usize> for Mask {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bits[index]
    }
}
