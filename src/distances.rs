use std::ops;

use rustc_hash::FxHashMap;

use crate::{cell::Cell, Grid};

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

    pub fn path_to(&self, goal: Cell, grid: &Grid) -> Self {
        let mut current = goal;

        let mut breadcrumbs = Self::new(self.root);
        breadcrumbs.insert(current, self[current]);

        while current != self.root {
            for linked in grid.links(current) {
                if self[linked] < self[current] {
                    breadcrumbs.insert(linked, self[linked]);
                    current = linked;
                    break;
                }
            }
        }

        breadcrumbs
    }
}

impl ops::Index<Cell> for Distances {
    type Output = usize;

    fn index(&self, index: Cell) -> &Self::Output {
        &self.distances[&index]
    }
}
