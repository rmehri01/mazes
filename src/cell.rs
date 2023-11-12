#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegularCell {
    pub row: isize,
    pub col: isize,
}

impl RegularCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolarCell {
    pub row: isize,
    pub col: isize,
}

impl PolarCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexCell {
    pub row: isize,
    pub col: isize,
}

impl HexCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn north_diagonal_row(&self) -> isize {
        if self.col % 2 == 0 {
            self.row - 1
        } else {
            self.row
        }
    }
    pub fn south_diagonal_row(&self) -> isize {
        if self.col % 2 == 0 {
            self.row
        } else {
            self.row + 1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TriangleCell {
    pub row: isize,
    pub col: isize,
}

impl TriangleCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn is_upright(&self) -> bool {
        (self.row + self.col) % 2 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeightedCell {
    pub row: isize,
    pub col: isize,
    pub weight: usize,
}

impl WeightedCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self {
            row,
            col,
            weight: 1,
        }
    }
}

pub trait CellKind
where
    Self: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash,
{
    fn row(&self) -> isize;
    fn weight(&self) -> usize {
        1
    }
}

impl CellKind for RegularCell {
    fn row(&self) -> isize {
        self.row
    }
}

impl CellKind for PolarCell {
    fn row(&self) -> isize {
        self.row
    }
}

impl CellKind for HexCell {
    fn row(&self) -> isize {
        self.row
    }
}

impl CellKind for TriangleCell {
    fn row(&self) -> isize {
        self.row
    }
}

impl CellKind for WeightedCell {
    fn row(&self) -> isize {
        self.row
    }

    fn weight(&self) -> usize {
        self.weight
    }
}
