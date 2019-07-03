
use crate::types::Size;
use std::iter;
pub struct Lines<'a, T, U> {
    items: &'a [T],
    size: Size<U>,
    base_width: usize,
    current: usize,
}

impl<'a, T, U> Lines<'a, T, U> {
    pub fn new(items: &'a [T], size: Size<U>, base_width: usize) -> Self {
        Self {
            items,
            size,
            base_width,
            current: 0,
        }
    }
}

impl<'a, T, U> iter::Iterator for Lines<'a, T, U> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size.height {
            None
        } else {
            let start = self.current * self.base_width;
            let result = Some(&self.items[start..start + self.size.width]);
            self.current += 1;
            result
        }
    }
}