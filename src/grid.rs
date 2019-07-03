use crate::index_range::IndexRange;
use crate::slice2d::{Slice2D, Slice2DMut};
use crate::types::{Index, Size};
use std::ops;

pub mod lines;

pub trait Grid<T, U>: ops::Index<Index<U>> {
    fn size(&self) -> Size<U>;

    fn get(&self, index: Index<U>) -> Option<&T>;

    fn as_slice2d(&self) -> Slice2D<T, U>;

    fn line(&self, y: usize) -> Option<&[T]>;

    fn lines(&self) -> lines::Lines<T, U>;

    fn crop(&self, range: impl IndexRange<U>) -> Slice2D<T, U>;
}

pub trait GridMut<T, U>: Grid<T, U> + ops::IndexMut<Index<U>> {
    fn get_mut(&mut self, index: Index<U>) -> Option<&mut T>;

    fn as_slice2d_mut(&mut self) -> Slice2DMut<T, U>;

    fn line_mut(&mut self, y: usize) -> Option<&mut [T]>;

    fn crop_mut(&mut self, range: impl IndexRange<U>) -> Slice2DMut<T, U>;
}
