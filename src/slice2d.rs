use crate::grid::Grid;
use crate::index_range::IndexRange;
use crate::types::{Index, Size};
use std::ops;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Slice2D<'a, T, U> {
    grid: Grid<&'a [T], U>,
}

impl<'a, T, U> Slice2D<'a, T, U> {
    pub fn new(items: &'a [T], size: Size<U>, base_width: usize) -> Self {
        Self {
            grid: Grid::new(items, size, base_width),
        }
    }

    pub fn size(&self) -> Size<U> {
        self.grid.size()
    }

    pub fn index_range(&self) -> ops::Range<Index<U>> {
        self.grid.index_range()
    }

    pub fn indices(&self) -> impl Iterator<Item = Index<U>> {
        use euclid_ext::Points;
        self.index_range().points()
    }

    pub fn get(&self, index: Index<U>) -> Option<&T> {
        self.grid.get(index)
    }

    pub fn as_slice2d(&self) -> Slice2D<T, U> {
        self.grid.as_slice2d()
    }

    pub fn line(&self, y: usize) -> Option<&[T]> {
        self.grid.line(y)
    }

    pub fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.grid.lines()
    }

    pub fn crop(&self, range: impl IndexRange<U>) -> Slice2D<T, U> {
        self.grid.crop(range)
    }
}

impl<'a, T, U> ops::Index<Index<U>> for Slice2D<'a, T, U> {
    type Output = T;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.grid.index(index)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Slice2DMut<'a, T, U> {
    grid: Grid<&'a mut [T], U>,
}

impl<'a, T, U> Slice2DMut<'a, T, U> {
    pub fn new(items: &'a mut [T], size: Size<U>, base_width: usize) -> Self {
        Self {
            grid: Grid::new(items, size, base_width),
        }
    }

    pub fn size(&self) -> Size<U> {
        self.grid.size()
    }

    pub fn index_range(&self) -> ops::Range<Index<U>> {
        self.grid.index_range()
    }

    pub fn indices(&self) -> impl Iterator<Item = Index<U>> {
        use euclid_ext::Points;
        self.index_range().points()
    }

    pub fn get(&self, index: Index<U>) -> Option<&T> {
        self.grid.get(index)
    }

    pub fn as_slice2d(&self) -> Slice2D<T, U> {
        self.grid.as_slice2d()
    }

    pub fn line(&self, y: usize) -> Option<&[T]> {
        self.grid.line(y)
    }

    pub fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.grid.lines()
    }

    pub fn crop(&self, range: impl IndexRange<U>) -> Slice2D<T, U> {
        self.grid.crop(range)
    }

    pub fn get_mut(&mut self, index: Index<U>) -> Option<&mut T> {
        self.grid.get_mut(index)
    }

    pub fn as_slice2d_mut(&mut self) -> Slice2DMut<T, U> {
        self.grid.as_slice2d_mut()
    }

    pub fn line_mut(&mut self, y: usize) -> Option<&mut [T]> {
        self.grid.line_mut(y)
    }

    pub fn crop_mut(&mut self, range: impl IndexRange<U>) -> Slice2DMut<T, U> {
        self.grid.crop_mut(range)
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
