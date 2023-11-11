#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    pub row: isize,
    pub col: isize,
}

pub trait CellKind
where
    Self: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash,
{
}

impl CellKind for Cell {}
