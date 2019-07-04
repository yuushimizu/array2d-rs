pub mod grid;
pub mod index_range;
pub mod slice2d;
pub mod types;

use euclid;
use index_range::IndexRange;
use std::ops;

pub use slice2d::{Slice2D, Slice2DMut};
pub use types::{Index, Size};

/// ```
/// use array2d::{Array2D, Slice2D, Slice2DMut, Size, Index};
/// struct Space;
/// let size = Size::<Space>::new(48, 32);
/// let mut a = Array2D::new(size, 0);
/// let mut s = a.as_slice2d_mut();
/// s[Index::new(3, 8)] = 123;
/// let sl = a.as_slice2d();
/// assert_eq!(123, sl[Index::new(3, 8)]);
/// let mut c = a.crop_mut(Index::new(10, 12)..Index::new(20, 24));
/// c[Index::new(3, 5)] = 1000;
/// c.crop(..Index::new(3, 3));
/// assert_eq!(1000, a[Index::new(13, 17)]);
/// assert_eq!(32, a.lines().count());
/// assert_eq!(48, a.lines().next().unwrap().len());
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Array2D<T, U = euclid::UnknownUnit> {
    grid: grid::Grid<Vec<T>, U>,
}

impl<T, U> Array2D<T, U> {
    pub fn new(size: Size<U>, initial_value: T) -> Array2D<T, U>
    where
        T: Clone,
    {
        Self {
            grid: grid::Grid::new(vec![initial_value; size.area()], size, size.width),
        }
    }

    pub fn size(&self) -> Size<U> {
        self.grid.size()
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
