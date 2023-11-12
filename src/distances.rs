use std::ops;

use rustc_hash::FxHashMap;

use crate::{grid::Grid, kind::Kind};

pub struct Distances<K: Kind> {
    root: K::Cell,
    distances: FxHashMap<K::Cell, usize>,
}

impl<K: Kind> Distances<K> {
    pub fn new(root: K::Cell) -> Self {
        Self {
            root,
            distances: FxHashMap::from_iter([(root, 0)]),
        }
    }

    pub fn get(&self, cell: &K::Cell) -> Option<usize> {
        self.distances.get(cell).copied()
    }

    pub fn insert(&mut self, cell: K::Cell, distance: usize) {
        self.distances.insert(cell, distance);
    }

    // TODO: does this work
    pub fn path_to(&self, goal: K::Cell, grid: &Grid<K>) -> Self {
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

    pub fn max(&self) -> (K::Cell, usize) {
        self.distances
            .iter()
            .max_by_key(|(_, dist)| **dist)
            .map(|(cell, dist)| (*cell, *dist))
            .expect("distances should be non-empty")
    }
}

impl<K: Kind> ops::Index<K::Cell> for Distances<K> {
    type Output = usize;

    fn index(&self, index: K::Cell) -> &Self::Output {
        &self.distances[&index]
    }
}
