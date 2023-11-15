use rand::{seq::SliceRandom, Rng};
use rustc_hash::FxHashMap;

use crate::{
    cell::WeaveCell,
    grid::Grid,
    kind::{Kind, Weave},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SetId(usize);

struct State<K: Kind> {
    grid: Grid<K>,
    neighbours: Vec<(K::Cell, K::Cell)>,
    set_for_cell: FxHashMap<K::Cell, SetId>,
    cells_in_set: FxHashMap<SetId, Vec<K::Cell>>,
}

impl<K: Kind> State<K> {
    fn new(grid: Grid<K>) -> Self {
        let neighbours = grid.neighbouring_cells();
        let mut set_for_cell = FxHashMap::default();
        let mut cells_in_set = FxHashMap::default();

        for cell in grid.cells() {
            let id = SetId(set_for_cell.len());

            set_for_cell.insert(cell, id);
            cells_in_set.insert(id, vec![cell]);
        }

        Self {
            grid,
            neighbours,
            set_for_cell,
            cells_in_set,
        }
    }

    fn can_merge(&self, left: K::Cell, right: K::Cell) -> bool {
        self.set_for_cell[&left] != self.set_for_cell[&right]
    }

    fn merge(&mut self, left: K::Cell, right: K::Cell) {
        self.grid.link(left, right);

        let left_id = self.set_for_cell[&left];
        let right_id = self.set_for_cell.get(&right);
        let right_cells = match right_id {
            Some(right_id) => self
                .cells_in_set
                .remove(right_id)
                .expect("right id to be found"),
            None => vec![right],
        };
        for cell in &right_cells {
            self.set_for_cell.insert(*cell, left_id);
        }

        self.cells_in_set
            .get_mut(&left_id)
            .expect("left id to be found")
            .extend(right_cells);
    }
}

impl State<Weave> {
    fn add_crossing(&mut self, cell: WeaveCell) {
        if let Some(((east, west), (north, south))) =
            (self.grid.east(cell).zip(self.grid.west(cell)))
                .zip(self.grid.north(cell).zip(self.grid.south(cell)))
        {
            if self.grid.links(cell).next().is_none()
                && self.can_merge(east, west)
                && self.can_merge(north, south)
            {
                self.neighbours
                    .retain(|(left, right)| *left != cell && *right != cell);

                if rand::random() {
                    self.merge(west, cell);
                    self.merge(cell, east);

                    let WeaveCell::Over(over) = cell else {
                        panic!("expected an over cell")
                    };
                    let under = self.grid.tunnel_under(north, over, south);
                    self.merge(north, under);
                    self.merge(under, south);
                } else {
                    self.merge(north, cell);
                    self.merge(cell, south);

                    let WeaveCell::Over(over) = cell else {
                        panic!("expected an over cell")
                    };
                    let under = self.grid.tunnel_under(west, over, east);
                    self.merge(west, under);
                    self.merge(under, east);
                }
            }
        }
    }
}

impl<K: Kind> Grid<K> {
    pub fn kruskals(self) -> Self {
        let mut state = State::new(self);
        state.neighbours.shuffle(&mut rand::thread_rng());

        while let Some((left, right)) = state.neighbours.pop() {
            if state.can_merge(left, right) {
                state.merge(left, right);
            }
        }

        state.grid
    }
}

impl Grid<Weave> {
    pub fn kruskals_better_weave(mut self) -> Self {
        self.get_kind_mut().is_preconfigured = true;

        let mut state = State::new(self);
        for _ in 0..state.grid.size() {
            state.add_crossing(WeaveCell::new(
                rand::thread_rng().gen_range(1..state.grid.num_rows() - 2) as isize,
                rand::thread_rng().gen_range(1..state.grid.num_cols() - 2) as isize,
            ));
        }

        state.neighbours.shuffle(&mut rand::thread_rng());

        while let Some((left, right)) = state.neighbours.pop() {
            if state.can_merge(left, right) {
                state.merge(left, right);
            }
        }

        state.grid
    }
}
