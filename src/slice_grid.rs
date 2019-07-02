use crate::grid::{Crop, CropMut, Grid, GridMut};
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

pub(crate) struct SliceGrid<T: AsSlice, U> {
    items: T,
    size: Size<U>,
    base_line_size: usize,
}

impl<T: AsSlice, U> SliceGrid<T, U> {
    pub fn new(items: T, size: Size<U>, base_line_size: usize) -> Self {
        Self {
            items,
            size,
            base_line_size,
        }
    }

    fn index_at_unchecked(&self, index: Index<U>) -> usize {
        index.y * self.base_line_size + index.x
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

impl<T: AsSlice, U> Crop<T::Item, U, ops::Range<Index<U>>> for SliceGrid<T, U> {
    fn crop(&self, range: ops::Range<Index<U>>) -> Slice2D<T::Item, U> {
        let start = self.clamp_index(range.start);
        let end = self.clamp_index(range.end);
        Slice2D::new(
            &self.items.as_slice()[self.index_at_unchecked(start)..self.index_at_unchecked(end)],
            (end - start).to_size(),
            self.base_line_size,
        )
    }
}

impl<T: AsSlice, U> Grid<T::Item, U> for SliceGrid<T, U> {
    fn size(&self) -> Size<U> {
        self.size
    }

    fn get(&self, index: Index<U>) -> Option<&T::Item> {
        self.index_at(index)
            .map(|index| &self.items.as_slice()[index])
    }

    fn as_slice2d(&self) -> Slice2D<T::Item, U> {
        Slice2D::new(self.items.as_slice(), self.size, self.base_line_size)
    }

    fn line(&self, y: usize) -> Option<&[T::Item]> {
        self.index_at(Index::new(0, y))
            .map(|start| &self.items.as_slice()[start..start + self.size.width])
    }
}

impl<T: AsMutSlice, U> CropMut<T::Item, U, ops::Range<Index<U>>> for SliceGrid<T, U> {
    fn crop_mut(&mut self, range: ops::Range<Index<U>>) -> Slice2DMut<T::Item, U> {
        let start = self.clamp_index(range.start);
        let end = self.clamp_index(range.end);
        let items_range = self.index_at_unchecked(start)..self.index_at_unchecked(end);
        Slice2DMut::new(
            &mut self.items.as_mut_slice()[items_range],
            (end - start).to_size(),
            self.base_line_size,
        )
    }
}

impl<T: AsMutSlice, U> GridMut<T::Item, U> for SliceGrid<T, U> {
    fn get_mut(&mut self, index: Index<U>) -> Option<&mut T::Item> {
        self.index_at(index)
            .map(move |index| &mut self.items.as_mut_slice()[index])
    }

    fn as_slice2d_mut(&mut self) -> Slice2DMut<T::Item, U> {
        Slice2DMut::new(self.items.as_mut_slice(), self.size, self.base_line_size)
    }

    fn line_mut(&mut self, y: usize) -> Option<&mut [T::Item]> {
        self.index_at(Index::new(0, y))
            .map(move |start| &mut self.items.as_mut_slice()[start..start + self.size.width])
    }
}