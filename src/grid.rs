use crate::index_range::IndexRange;
use crate::slice2d::{Slice2D, Slice2DMut};
use crate::types::{Index, Size};
use std::ops;

pub mod lines;

pub trait Grid: ops::Index<Index<<Self as Grid>::Unit>> {
    type Item;

    type Unit;

    fn size(&self) -> Size<Self::Unit>;

    fn get(&self, index: Index<Self::Unit>) -> Option<&Self::Item>;

    fn as_slice2d(&self) -> Slice2D<Self::Item, Self::Unit>;

    fn line(&self, y: usize) -> Option<&[Self::Item]>;

    fn lines(&self) -> lines::Lines<Self> {
        lines::Lines::new(self)
    }

    fn crop(&self, range: impl IndexRange<Self::Unit>) -> Slice2D<Self::Item, Self::Unit>;
}

pub trait GridMut: Grid + ops::IndexMut<Index<<Self as Grid>::Unit>> {
    fn get_mut(&mut self, index: Index<Self::Unit>) -> Option<&mut Self::Item>;

    fn as_slice2d_mut(&mut self) -> Slice2DMut<Self::Item, Self::Unit>;

    fn line_mut(&mut self, y: usize) -> Option<&mut [Self::Item]>;

    //    fn lines_mut(&mut self) -> lines::LinesMut<T, U>;

    fn crop_mut(
        &mut self,
        range: impl IndexRange<Self::Unit>,
    ) -> Slice2DMut<Self::Item, Self::Unit>;
}
