pub mod as_slice;
mod lines;

use crate::index_range::IndexRange;
use crate::slice2d::{Slice2D, Slice2DMut};
use crate::types::{Index, Size};
use as_slice::{AsMutSlice, AsSlice};
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid<T: AsSlice, U> {
    items: T,
    size: Size<U>,
    base_width: usize,
}

impl<T: AsSlice, U> Grid<T, U> {
    pub fn new(items: T, size: Size<U>, base_width: usize) -> Self {
        Self {
            items,
            size,
            base_width,
        }
    }

    fn index_at_unchecked(&self, index: Index<U>) -> usize {
        index.y * self.base_width + index.x
    }

    fn in_bounds(&self, index: Index<U>) -> bool {
        euclid::TypedRect::new(Index::zero(), self.size).contains(&index)
    }

    fn index_at(&self, index: Index<U>) -> Option<usize> {
        if self.in_bounds(index) {
            Some(self.index_at_unchecked(index))
        } else {
            None
        }
    }

    fn debug_asserted_index_at(&self, index: Index<U>) -> usize {
        debug_assert!(
            self.in_bounds(index),
            "index out of bounds: index = {}, size = {}",
            index,
            self.size
        );
        self.index_at_unchecked(index)
    }

    fn clamp_index(&self, index: Index<U>) -> Index<U> {
        use euclid_ext::Map2D;
        (index, self.size()).map(|(n, size)| if n >= size { size } else { n })
    }

    pub fn size(&self) -> Size<U> {
        self.size
    }

    pub fn get(&self, index: Index<U>) -> Option<&T::Item> {
        self.index_at(index)
            .map(|index| &self.items.as_slice()[index])
    }

    pub fn as_slice2d(&self) -> Slice2D<T::Item, U> {
        Slice2D::new(self.items.as_slice(), self.size, self.base_width)
    }

    pub fn line(&self, y: usize) -> Option<&[T::Item]> {
        self.index_at(Index::new(0, y))
            .map(|start| &self.items.as_slice()[start..start + self.size.width])
    }

    pub fn lines(&self) -> impl Iterator<Item = &[T::Item]> {
        lines::Lines::new(&self)
    }

    pub fn crop(&self, range: impl IndexRange<U>) -> Slice2D<T::Item, U> {
        let range = range.normalize(self.size());
        let start = self.clamp_index(range.start);
        let end = self.clamp_index(range.end);
        Slice2D::new(
            &self.items.as_slice()[self.index_at_unchecked(start)..self.index_at_unchecked(end)],
            (end - start).to_size(),
            self.base_width,
        )
    }
}

impl<T: AsSlice, U> ops::Index<Index<U>> for Grid<T, U> {
    type Output = T::Item;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.items
            .as_slice()
            .index(self.debug_asserted_index_at(index))
    }
}

impl<T: AsMutSlice, U> ops::IndexMut<Index<U>> for Grid<T, U> {
    fn index_mut(&mut self, index: Index<U>) -> &mut Self::Output {
        let index = self.debug_asserted_index_at(index);
        self.items.as_mut_slice().index_mut(index)
    }
}

impl<T: AsMutSlice, U> Grid<T, U> {
    pub fn get_mut(&mut self, index: Index<U>) -> Option<&mut T::Item> {
        self.index_at(index)
            .map(move |index| &mut self.items.as_mut_slice()[index])
    }

    pub fn as_slice2d_mut(&mut self) -> Slice2DMut<T::Item, U> {
        Slice2DMut::new(self.items.as_mut_slice(), self.size, self.base_width)
    }

    pub fn line_mut(&mut self, y: usize) -> Option<&mut [T::Item]> {
        self.index_at(Index::new(0, y))
            .map(move |start| &mut self.items.as_mut_slice()[start..start + self.size.width])
    }

    pub fn crop_mut(&mut self, range: impl IndexRange<U>) -> Slice2DMut<T::Item, U> {
        let range = range.normalize(self.size());
        let start = self.clamp_index(range.start);
        let end = self.clamp_index(range.end);
        let items_range = self.index_at_unchecked(start)..self.index_at_unchecked(end);
        Slice2DMut::new(
            &mut self.items.as_mut_slice()[items_range],
            (end - start).to_size(),
            self.base_width,
        )
    }
}