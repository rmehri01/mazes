use rand::Rng;

use crate::{grid::Grid, kind::Regular};

impl Grid<Regular> {
    pub fn recursive_division(mut self) -> Self {
        for cell in self.cells() {
            self.neighbours(cell)
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|n| self.link(cell, n));
        }

        self.divide(0, 0, self.num_rows(), self.num_cols());

        self
    }

    fn divide(&mut self, row: isize, col: isize, height: usize, width: usize) {
        if height <= 1
            || width <= 1
            || height < 5 && width < 5 && rand::thread_rng().gen_range(0..4) == 0
        {
            return;
        }

        if height > width {
            self.divide_horizontally(row, col, height, width);
        } else {
            self.divide_vertically(row, col, height, width);
        }
    }

    fn divide_horizontally(&mut self, row: isize, col: isize, height: usize, width: usize) {
        let divide_south_of = rand::thread_rng().gen_range(0..height - 1);
        let passage_at = rand::thread_rng().gen_range(0..width);

        for x in 0..width {
            if passage_at != x {
                let cell = self
                    .get(row + divide_south_of as isize, col + x as isize)
                    .expect("cell to exist");
                let south = self.south(cell).expect("cell should have a south");
                self.unlink(cell, south);
            }
        }

        self.divide(row, col, divide_south_of + 1, width);
        self.divide(
            row + divide_south_of as isize + 1,
            col,
            height - divide_south_of - 1,
            width,
        );
    }

    fn divide_vertically(&mut self, row: isize, col: isize, height: usize, width: usize) {
        let divide_east_of = rand::thread_rng().gen_range(0..width - 1);
        let passage_at = rand::thread_rng().gen_range(0..height);

        for y in 0..height {
            if passage_at != y {
                let cell = self
                    .get(row + y as isize, col + divide_east_of as isize)
                    .expect("cell to exist");
                let east = self.east(cell).expect("cell should have a south");
                self.unlink(cell, east);
            }
        }

        self.divide(row, col, height, divide_east_of + 1);
        self.divide(
            row,
            col + divide_east_of as isize + 1,
            height,
            width - divide_east_of - 1,
        );
    }
}
