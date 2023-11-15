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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaveCell {
    Over(OverCell),
    Under(UnderCell),
}

impl WeaveCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self::Over(OverCell::new(row, col))
    }

    pub fn col(&self) -> isize {
        match self {
            Self::Over(o) => o.col,
            Self::Under(u) => u.over.col,
        }
    }

    /// Returns `true` if the weave cell is [`Over`].
    ///
    /// [`Over`]: WeaveCell::Over
    #[must_use]
    pub fn is_over(&self) -> bool {
        matches!(self, Self::Over(..))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverCell {
    pub row: isize,
    pub col: isize,
}

impl OverCell {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnderCell {
    pub over: OverCell,
}

impl UnderCell {
    pub fn new(over: OverCell) -> Self {
        Self { over }
    }
}

pub trait CellKind
where
    Self: std::fmt::Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash,
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

impl CellKind for WeaveCell {
    fn row(&self) -> isize {
        match self {
            Self::Over(o) => o.row,
            Self::Under(u) => u.over.row,
        }
    }
}
