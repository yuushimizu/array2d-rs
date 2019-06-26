use crate::slice2d::{Slice2D, Slice2DMut};
use crate::types::{Index, Size};
use euclid;
use std::ops;

pub trait Clip<T, U, R> {
    fn clip(&self, range: R) -> Slice2D<T, U>;
}

pub trait ClipMut<T, U, R> {
    fn clip_mut(&mut self, range: R) -> Slice2DMut<T, U>;
}

pub trait Grid<T, U>: ops::Index<Index<U>> + Clip<T, U, ops::Range<Index<U>>> {
    fn size(&self) -> Size<U>;

    fn get(&self, index: Index<U>) -> Option<&T>;

    fn as_slice2d(&self) -> Slice2D<T, U>;

    fn line(&self, y: usize) -> Option<&[T]>;
}

impl<T, U, G: Grid<T, U>> Clip<T, U, ops::RangeInclusive<Index<U>>> for G {
    fn clip(&self, range: ops::RangeInclusive<Index<U>>) -> Slice2D<T, U> {
        self.clip(*range.start()..*range.end() + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: Grid<T, U>> Clip<T, U, ops::RangeFrom<Index<U>>> for G {
    fn clip(&self, range: ops::RangeFrom<Index<U>>) -> Slice2D<T, U> {
        self.clip(range.start..Index::zero() + self.size())
    }
}

impl<T, U, G: Grid<T, U>> Clip<T, U, ops::RangeTo<Index<U>>> for G {
    fn clip(&self, range: ops::RangeTo<Index<U>>) -> Slice2D<T, U> {
        self.clip(Index::zero()..range.end)
    }
}

impl<T, U, G: Grid<T, U>> Clip<T, U, ops::RangeToInclusive<Index<U>>> for G {
    fn clip(&self, range: ops::RangeToInclusive<Index<U>>) -> Slice2D<T, U> {
        self.clip(Index::zero()..range.end + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: Grid<T, U>> Clip<T, U, ops::RangeFull> for G {
    fn clip(&self, _range: ops::RangeFull) -> Slice2D<T, U> {
        self.as_slice2d()
    }
}

impl<T, U, G: Grid<T, U>> Clip<T, U, euclid::TypedBox2D<usize, U>> for G {
    fn clip(&self, b: euclid::TypedBox2D<usize, U>) -> Slice2D<T, U> {
        self.clip(b.min..b.max)
    }
}

impl<T, U, G: Grid<T, U>> Clip<T, U, euclid::TypedRect<usize, U>> for G {
    fn clip(&self, rect: euclid::TypedRect<usize, U>) -> Slice2D<T, U> {
        self.clip(rect.origin..rect.bottom_right())
    }
}

pub trait GridMut<T, U>:
    Grid<T, U> + ops::IndexMut<Index<U>> + ClipMut<T, U, ops::Range<Index<U>>>
{
    fn get_mut(&mut self, index: Index<U>) -> Option<&mut T>;

    fn as_slice2d_mut(&mut self) -> Slice2DMut<T, U>;

    fn line_mut(&mut self, y: usize) -> Option<&mut [T]>;
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, ops::RangeInclusive<Index<U>>> for G {
    fn clip_mut(&mut self, range: ops::RangeInclusive<Index<U>>) -> Slice2DMut<T, U> {
        self.clip_mut(*range.start()..*range.end() + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, ops::RangeFrom<Index<U>>> for G {
    fn clip_mut(&mut self, range: ops::RangeFrom<Index<U>>) -> Slice2DMut<T, U> {
        self.clip_mut(range.start..Index::zero() + self.size())
    }
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, ops::RangeTo<Index<U>>> for G {
    fn clip_mut(&mut self, range: ops::RangeTo<Index<U>>) -> Slice2DMut<T, U> {
        self.clip_mut(Index::zero()..range.end)
    }
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, ops::RangeToInclusive<Index<U>>> for G {
    fn clip_mut(&mut self, range: ops::RangeToInclusive<Index<U>>) -> Slice2DMut<T, U> {
        self.clip_mut(Index::zero()..range.end + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, ops::RangeFull> for G {
    fn clip_mut(&mut self, _range: ops::RangeFull) -> Slice2DMut<T, U> {
        self.as_slice2d_mut()
    }
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, euclid::TypedBox2D<usize, U>> for G {
    fn clip_mut(&mut self, b: euclid::TypedBox2D<usize, U>) -> Slice2DMut<T, U> {
        self.clip_mut(b.min..b.max)
    }
}

impl<T, U, G: GridMut<T, U>> ClipMut<T, U, euclid::TypedRect<usize, U>> for G {
    fn clip_mut(&mut self, rect: euclid::TypedRect<usize, U>) -> Slice2DMut<T, U> {
        self.clip_mut(rect.origin..rect.bottom_right())
    }
}