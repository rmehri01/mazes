use std::{fs, ops, path::Path};

pub struct Mask {
    rows: usize,
    cols: usize,
    bits: Vec<Vec<bool>>,
}

impl Mask {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            bits: vec![vec![true; cols]; rows],
        }
    }

    pub fn from_txt(path: impl AsRef<Path>) -> Self {
        let file = fs::read_to_string(path).expect("file should be found");
        let bits: Vec<Vec<bool>> = file
            .lines()
            .map(|line| line.chars().map(|c| c != 'X').collect())
            .collect();

        Self {
            rows: bits.len(),
            cols: bits[0].len(),
            bits,
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
