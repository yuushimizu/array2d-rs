use crate::grid::{Grid, GridMut, Crop, CropMut};
use crate::slice_grid::{SliceGrid};
use crate::types::{Index, Size};
use std::ops;

pub struct Slice2D<'a, T, U> {
    grid: SliceGrid<&'a [T], U>,
}

impl<'a, T, U> Slice2D<'a, T, U> {
    pub fn new(items: &'a [T], size: Size<U>, base_line_size: usize) -> Self {
        Self {
            grid: SliceGrid::new(items, size, base_line_size),
        }
    }
}

impl<'a, T, U> ops::Index<Index<U>> for Slice2D<'a, T, U> {
    type Output = T;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.grid.index(index)
    }
}

impl<'a, T, U> Crop<T, U, ops::Range<Index<U>>> for Slice2D<'a, T, U> {
    fn crop(&self, range: ops::Range<Index<U>>) -> Slice2D<T, U> {
        self.grid.crop(range)
    }
}

impl<'a, T, U> Grid<T, U> for Slice2D<'a, T, U> {
    fn size(&self) -> Size<U> {
        self.grid.size()
    }

    fn get(&self, index: Index<U>) -> Option<&T> {
        self.grid.get(index)
    }

    fn as_slice2d(&self) -> Slice2D<T, U> {
        self.grid.as_slice2d()
    }

    fn line(&self, y: usize) -> Option<&[T]> {
        self.grid.line(y)
    }
}

pub struct Slice2DMut<'a, T, U> {
    grid: SliceGrid<&'a mut [T], U>,
}

impl<'a, T, U> Slice2DMut<'a, T, U> {
    pub fn new(items: &'a mut [T], size: Size<U>, base_line_size: usize) -> Self {
        Self {
            grid: SliceGrid::new(items, size, base_line_size),
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

impl<'a, T, U> Crop<T, U, ops::Range<Index<U>>> for Slice2DMut<'a, T, U> {
    fn crop(&self, range: ops::Range<Index<U>>) -> Slice2D<T, U> {
        self.grid.crop(range)
    }
}

impl<'a, T, U> Grid<T, U> for Slice2DMut<'a, T, U> {
    fn size(&self) -> Size<U> {
        self.grid.size()
    }

    fn get(&self, index: Index<U>) -> Option<&T> {
        self.grid.get(index)
    }

    fn as_slice2d(&self) -> Slice2D<T, U> {
        self.grid.as_slice2d()
    }

    fn line(&self, y: usize) -> Option<&[T]> {
        self.grid.line(y)
    }
}

impl<'a, T, U> CropMut<T, U, ops::Range<Index<U>>> for Slice2DMut<'a, T, U> {
    fn crop_mut(&mut self, range: ops::Range<Index<U>>) -> Slice2DMut<T, U> {
        self.grid.crop_mut(range)
    }
}

impl<'a, T, U> GridMut<T, U> for Slice2DMut<'a, T, U> {
    fn get_mut(&mut self, index: Index<U>) -> Option<&mut T> {
        self.grid.get_mut(index)
    }

    fn as_slice2d_mut(&mut self) -> Slice2DMut<T, U> {
        self.grid.as_slice2d_mut()
    }

    fn line_mut(&mut self, y: usize) -> Option<&mut [T]> {
        self.grid.line_mut(y)
    }
}