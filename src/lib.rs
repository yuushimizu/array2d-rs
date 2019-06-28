use euclid;
use std::ops;

pub mod grid;
pub mod slice2d;
pub mod types;

pub(crate) mod slice_grid;

pub use grid::{Clip, ClipMut, Grid, GridMut};
pub use slice2d::{Slice2D, Slice2DMut};
pub use types::{Index, Size};

/// ```
/// use array2d::{Array2D, Slice2D, Slice2DMut, Grid, GridMut, Clip, ClipMut, Size, Index};
/// struct Space;
/// let size = Size::<Space>::new(48, 32);
/// let mut a = Array2D::new(size, 0);
/// let mut s = a.as_slice2d_mut();
/// s[Index::new(3, 8)] = 123;
/// let sl = a.as_slice2d();
/// assert_eq!(123, sl[Index::new(3, 8)]);
/// let mut c = a.clip_mut(Index::new(10, 12)..Index::new(20, 24));
/// c[Index::new(3, 5)] = 1000;
/// c.clip(..Index::new(3, 3));
/// assert_eq!(1000, a[Index::new(13, 17)]);
/// ```
pub struct Array2D<T, U = euclid::UnknownUnit> {
    grid: slice_grid::SliceGrid<Vec<T>, U>,
}

impl<T, U> Array2D<T, U> {
    pub fn new(size: Size<U>, initial_value: T) -> Array2D<T, U>
    where
        T: Clone,
    {
        Self {
            grid: slice_grid::SliceGrid::new(vec![initial_value; size.area()], size, size.width),
        }
    }
}

impl<T, U> ops::Index<Index<U>> for Array2D<T, U> {
    type Output = T;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.grid.index(index)
    }
}

impl<T, U> ops::IndexMut<Index<U>> for Array2D<T, U> {
    fn index_mut(&mut self, index: Index<U>) -> &mut Self::Output {
        self.grid.index_mut(index)
    }
}

impl<T, U> Clip<T, U, ops::Range<Index<U>>> for Array2D<T, U> {
    fn clip(&self, range: ops::Range<Index<U>>) -> Slice2D<T, U> {
        self.grid.clip(range)
    }
}

impl<T, U> Grid<T, U> for Array2D<T, U> {
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

impl<T, U> ClipMut<T, U, ops::Range<Index<U>>> for Array2D<T, U> {
    fn clip_mut(&mut self, range: ops::Range<Index<U>>) -> Slice2DMut<T, U> {
        self.grid.clip_mut(range)
    }
}

impl<T, U> GridMut<T, U> for Array2D<T, U> {
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