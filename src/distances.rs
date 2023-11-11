use std::ops;

use rustc_hash::FxHashMap;

use crate::{cell::Cell, grid::GridKind, Grid};

pub struct Distances {
    root: Cell,
    distances: FxHashMap<Cell, usize>,
}

impl Distances {
    pub fn new(root: Cell) -> Self {
        Self {
            root,
            distances: FxHashMap::from_iter([(root, 0)]),
        }
    }

    pub fn get(&self, cell: &Cell) -> Option<usize> {
        self.distances.get(cell).copied()
    }

    pub fn insert(&mut self, cell: Cell, distance: usize) {
        self.distances.insert(cell, distance);
    }

    // TODO: does this work
    pub fn path_to(&self, goal: Cell, grid: &Grid<impl GridKind>) -> Self {
        let mut current = goal;

        let mut breadcrumbs = Self::new(self.root);
        breadcrumbs.insert(current, self[current]);

        while current != self.root {
            let closer = grid
                .links(current)
                .find(|linked| self[*linked] < self[current])
                .expect("at least one link to the current cell should be closer to the root");
            breadcrumbs.insert(closer, self[closer]);
            current = closer;
        }

        breadcrumbs
    }

    pub fn max(&self) -> (Cell, usize) {
        self.distances
            .iter()
            .max_by_key(|(_, dist)| **dist)
            .map(|(cell, dist)| (*cell, *dist))
            .expect("distances should be non-empty")
    }
}

impl ops::Index<Cell> for Distances {
    type Output = usize;

    fn index(&self, index: Cell) -> &Self::Output {
        &self.distances[&index]
    }
}
