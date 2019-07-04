use std::iter;
use super::{SliceGrid, AsSlice};

#[derive(Debug, Clone)]
pub struct Lines<'a, T: AsSlice + 'a, U> {
    grid: &'a SliceGrid<T, U>,
    current: usize,
}

impl<'a, T: AsSlice + 'a, U> Lines<'a, T, U> {
    pub fn new(grid: &'a SliceGrid<T, U>) -> Self {
        Self { grid, current: 0 }
    }
}

impl<'a, T: AsSlice + 'a, U> iter::Iterator for Lines<'a, T, U> {
    type Item = &'a [T::Item];

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
