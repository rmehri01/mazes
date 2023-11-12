#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegularCell {
    pub row: isize,
    pub col: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolarCell {
    pub row: isize,
    pub col: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexCell {
    pub row: isize,
    pub col: isize,
}

impl HexCell {
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
    pub fn is_upright(&self) -> bool {
        (self.row + self.col) % 2 == 0
    }
}

pub trait CellKind
where
    Self: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash,
{
    fn row(&self) -> isize;
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
