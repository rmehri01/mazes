use rand::{seq::SliceRandom, Rng};
use rustc_hash::FxHashMap;

use crate::{cell::RegularCell, grid::Grid, kind::Regular};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SetId(usize);

struct RowState {
    set_for_cell: FxHashMap<RegularCell, SetId>,
    cells_in_set: FxHashMap<SetId, Vec<RegularCell>>,
    next_set: SetId,
}

impl RowState {
    fn new(next_set: SetId) -> Self {
        Self {
            set_for_cell: FxHashMap::default(),
            cells_in_set: FxHashMap::default(),
            next_set,
        }
    }

    fn record(&mut self, set: SetId, cell: RegularCell) {
        self.set_for_cell.insert(cell, set);
        self.cells_in_set.entry(set).or_default().push(cell);
    }

    fn set_for(&mut self, cell: RegularCell) -> SetId {
        if self.set_for_cell.get(&cell).is_none() {
            self.record(self.next_set, cell);
            self.next_set = SetId(self.next_set.0 + 1);
        }

        self.set_for_cell[&cell]
    }

    fn merge(&mut self, left_set: SetId, right_set: SetId) {
        let right = self
            .cells_in_set
            .remove(&right_set)
            .expect("right set id to be found");
        for cell in &right {
            self.set_for_cell.insert(*cell, left_set);
        }

        self.cells_in_set
            .get_mut(&left_set)
            .expect("left set id to be found")
            .extend(right);
    }

    fn next(&self) -> Self {
        Self::new(self.next_set)
    }

    fn each_set(&self) -> impl Iterator<Item = (&SetId, &Vec<RegularCell>)> {
        self.cells_in_set.iter()
    }
}

impl Grid<Regular> {
    pub fn ellers(mut self) -> Self {
        let mut row_state = RowState::new(SetId(0));

        for row in self.rows() {
            for &cell in &row {
                if let Some(west) = self.west(cell) {
                    let set = row_state.set_for(cell);
                    let prior_set = row_state.set_for(west);

                    let should_link =
                        set != prior_set && (self.south(cell).is_none() || rand::random());
                    if should_link {
                        self.link(cell, west);
                        row_state.merge(prior_set, set);
                    }
                }
            }

            if self.south(row[0]).is_some() {
                let mut next_row = row_state.next();

                for (_, list) in row_state.each_set() {
                    let mut list = list.clone();
                    list.shuffle(&mut rand::thread_rng());

                    for (idx, cell) in list.into_iter().enumerate() {
                        if idx == 0 || rand::thread_rng().gen_range(0..3) == 0 {
                            let south = self.south(cell).expect("south to exist");
                            self.link(cell, south);
                            next_row.record(row_state.set_for_cell[&cell], south);
                        }
                    }
                }

                row_state = next_row;
            }
        }

        self
    }
}
