use rand::seq::SliceRandom;

use crate::{
    grid::Grid,
    kind::{Hex, Regular, ThreeD},
};

impl Grid<Regular> {
    pub fn sidewinder(mut self) -> Self {
        for row in self.rows() {
            let mut run = Vec::new();

            for cell in row {
                run.push(cell);

                let at_east_boundary = self.east(cell).is_none();
                let at_north_boundary = self.north(cell).is_none();

                let should_close = at_east_boundary || (!at_north_boundary && rand::random());

                if should_close {
                    let member = run
                        .choose(&mut rand::thread_rng())
                        .copied()
                        .expect("run should be non-empty");

                    if let Some(north) = self.north(member) {
                        self.link(member, north);
                    }
                    run.clear();
                } else {
                    let east = self
                        .east(cell)
                        .expect("east should always exist when not closing a run");
                    self.link(cell, east);
                }
            }
        }

        self
    }
}

impl Grid<Hex> {
    pub fn sidewinder(mut self) -> Self {
        for row in self.rows() {
            let mut run = Vec::new();

            for cell in row {
                run.push(cell);

                let at_east_boundary = self.get_next_in_row(cell).is_none();
                let at_north_boundary = self.north(cell).is_none();

                let should_close = at_east_boundary || (!at_north_boundary && rand::random());

                if should_close {
                    let member = run
                        .choose(&mut rand::thread_rng())
                        .copied()
                        .expect("run should be non-empty");

                    if let Some(north) = self.north(member) {
                        self.link(member, north);
                    }
                    run.clear();
                } else {
                    let east = self
                        .get_next_in_row(cell)
                        .expect("east should always exist when not closing a run");
                    self.link(cell, east);
                }
            }
        }

        self
    }
}

impl Grid<ThreeD> {
    pub fn sidewinder(mut self) -> Self {
        for row in self.rows() {
            let mut run = Vec::new();

            for cell in row {
                run.push(cell);

                let at_east_boundary = self.east(cell).is_none();
                let at_north_boundary = self.north(cell).is_none();

                let should_close = at_east_boundary || (!at_north_boundary && rand::random());

                if should_close {
                    let member = run
                        .choose(&mut rand::thread_rng())
                        .copied()
                        .expect("run should be non-empty");

                    match (self.north(member), self.up(member)) {
                        (Some(north), Some(up)) => {
                            if rand::random() {
                                self.link(member, north);
                            } else {
                                self.link(member, up);
                            }
                        }
                        (None, Some(other)) | (Some(other), None) => self.link(member, other),
                        (None, None) => {}
                    }

                    run.clear();
                } else {
                    let east = self
                        .east(cell)
                        .expect("east should always exist when not closing a run");
                    self.link(cell, east);
                }
            }
        }

        self
    }
}
