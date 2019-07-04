use crate::grid::{Grid, GridMut};
use crate::index_range::IndexRange;
use crate::slice_grid::SliceGrid;
use crate::types::{Index, Size};
use std::ops;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Slice2D<'a, T, U> {
    grid: SliceGrid<&'a [T], U>,
}

impl<'a, T, U> Slice2D<'a, T, U> {
    pub fn new(items: &'a [T], size: Size<U>, base_width: usize) -> Self {
        Self {
            grid: SliceGrid::new(items, size, base_width),
        }
    }
}

impl<'a, T, U> ops::Index<Index<U>> for Slice2D<'a, T, U> {
    type Output = T;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.grid.index(index)
    }
}

impl<'a, T, U> Grid for Slice2D<'a, T, U> {
    type Item = T;

    type Unit = U;

    fn size(&self) -> Size<Self::Unit> {
        self.grid.size()
    }

    fn get(&self, index: Index<Self::Unit>) -> Option<&Self::Item> {
        self.grid.get(index)
    }

    fn as_slice2d(&self) -> Slice2D<Self::Item, Self::Unit> {
        self.grid.as_slice2d()
    }

    fn line(&self, y: usize) -> Option<&[Self::Item]> {
        self.grid.line(y)
    }

    fn crop(&self, range: impl IndexRange<Self::Unit>) -> Slice2D<Self::Item, Self::Unit> {
        self.grid.crop(range)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Slice2DMut<'a, T, U> {
    grid: SliceGrid<&'a mut [T], U>,
}

impl<'a, T, U> Slice2DMut<'a, T, U> {
    pub fn new(items: &'a mut [T], size: Size<U>, base_width: usize) -> Self {
        Self {
            grid: SliceGrid::new(items, size, base_width),
        }
    }
}

impl<'a, T, U> ops::Index<Index<U>> for Slice2DMut<'a, T, U> {
    type Output = T;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.grid.index(index)
    }
}

impl<'a, T, U> ops::IndexMut<Index<U>> for Slice2DMut<'a, T, U> {
    fn index_mut(&mut self, index: Index<U>) -> &mut Self::Output {
        self.grid.index_mut(index)
    }
}

impl<'a, T, U> Grid for Slice2DMut<'a, T, U> {
    type Item = T;

    type Unit = U;

    fn size(&self) -> Size<Self::Unit> {
        self.grid.size()
    }

    fn get(&self, index: Index<Self::Unit>) -> Option<&Self::Item> {
        self.grid.get(index)
    }

    fn as_slice2d(&self) -> Slice2D<Self::Item, Self::Unit> {
        self.grid.as_slice2d()
    }

    fn line(&self, y: usize) -> Option<&[Self::Item]> {
        self.grid.line(y)
    }

    fn crop(&self, range: impl IndexRange<Self::Unit>) -> Slice2D<Self::Item, Self::Unit> {
        self.grid.crop(range)
    }
}

impl<'a, T, U> GridMut for Slice2DMut<'a, T, U> {
    fn get_mut(&mut self, index: Index<Self::Unit>) -> Option<&mut Self::Item> {
        self.grid.get_mut(index)
    }

    fn as_slice2d_mut(&mut self) -> Slice2DMut<Self::Item, Self::Unit> {
        self.grid.as_slice2d_mut()
    }

    fn line_mut(&mut self, y: usize) -> Option<&mut [Self::Item]> {
        self.grid.line_mut(y)
    }

    fn crop_mut(
        &mut self,
        range: impl IndexRange<Self::Unit>,
    ) -> Slice2DMut<Self::Item, Self::Unit> {
        self.grid.crop_mut(range)
    }
}