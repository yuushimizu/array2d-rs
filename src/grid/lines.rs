
use crate::grid::Grid;
use std::iter;

pub struct Lines<'a, G: Grid + ?Sized> {
    grid: &'a G,
    current: usize,
}

impl<'a, G: Grid + ?Sized> Lines<'a, G> {
    pub fn new(grid: &'a G) -> Self {
        Self { grid, current: 0 }
    }
}

impl<'a, G: Grid + ?Sized> iter::Iterator for Lines<'a, G> {
    type Item = &'a [G::Item];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.grid.size().height {
            None
        } else {
            let line = self.grid.line(self.current);
            self.current += 1;
            line
        }
    }
}
