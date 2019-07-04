use crate::grid::{Grid, GridMut};
use crate::index_range::IndexRange;
use crate::slice2d::{Slice2D, Slice2DMut};
use crate::types::{Index, Size};
use std::ops;

pub(crate) trait AsSlice {
    type Item;

    fn as_slice(&self) -> &[Self::Item];
}

impl<T> AsSlice for &[T] {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> AsSlice for &mut [T] {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

pub(crate) trait AsMutSlice: AsSlice {
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<T> AsMutSlice for &mut [T] {
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}

impl<T> AsSlice for Vec<T> {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> AsMutSlice for Vec<T> {
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct SliceGrid<T: AsSlice, U> {
    items: T,
    size: Size<U>,
    base_width: usize,
}

impl<T: AsSlice, U> SliceGrid<T, U> {
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
}

impl<T: AsSlice, U> ops::Index<Index<U>> for SliceGrid<T, U> {
    type Output = T::Item;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.items
            .as_slice()
            .index(self.debug_asserted_index_at(index))
    }
}

impl<T: AsMutSlice, U> ops::IndexMut<Index<U>> for SliceGrid<T, U> {
    fn index_mut(&mut self, index: Index<U>) -> &mut Self::Output {
        let index = self.debug_asserted_index_at(index);
        self.items.as_mut_slice().index_mut(index)
    }
}

impl<T: AsSlice, U> Grid for SliceGrid<T, U> {
    type Item = T::Item;

    type Unit = U;

    fn size(&self) -> Size<Self::Unit> {
        self.size
    }

    fn get(&self, index: Index<Self::Unit>) -> Option<&Self::Item> {
        self.index_at(index)
            .map(|index| &self.items.as_slice()[index])
    }

    fn as_slice2d(&self) -> Slice2D<Self::Item, Self::Unit> {
        Slice2D::new(self.items.as_slice(), self.size, self.base_width)
    }

    fn line(&self, y: usize) -> Option<&[Self::Item]> {
        self.index_at(Index::new(0, y))
            .map(|start| &self.items.as_slice()[start..start + self.size.width])
    }

    fn crop(&self, range: impl IndexRange<Self::Unit>) -> Slice2D<Self::Item, Self::Unit> {
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

impl<T: AsMutSlice, U> GridMut for SliceGrid<T, U> {
    fn get_mut(&mut self, index: Index<Self::Unit>) -> Option<&mut Self::Item> {
        self.index_at(index)
            .map(move |index| &mut self.items.as_mut_slice()[index])
    }

    fn as_slice2d_mut(&mut self) -> Slice2DMut<Self::Item, Self::Unit> {
        Slice2DMut::new(self.items.as_mut_slice(), self.size, self.base_width)
    }

    fn line_mut(&mut self, y: usize) -> Option<&mut [Self::Item]> {
        self.index_at(Index::new(0, y))
            .map(move |start| &mut self.items.as_mut_slice()[start..start + self.size.width])
    }

    fn crop_mut(
        &mut self,
        range: impl IndexRange<Self::Unit>,
    ) -> Slice2DMut<Self::Item, Self::Unit> {
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