use crate::slice2d::{Slice2D, Slice2DMut};
use crate::types::{Index, Size};
use euclid;
use std::ops;

pub mod lines;

pub trait Crop<T, U, R> {
    fn crop(&self, range: R) -> Slice2D<T, U>;
}

pub trait CropMut<T, U, R>: Crop<T, U, R> {
    fn crop_mut(&mut self, range: R) -> Slice2DMut<T, U>;
}

pub trait Grid<T, U>: ops::Index<Index<U>> + Crop<T, U, ops::Range<Index<U>>> {
    fn size(&self) -> Size<U>;

    fn get(&self, index: Index<U>) -> Option<&T>;

    fn as_slice2d(&self) -> Slice2D<T, U>;

    fn line(&self, y: usize) -> Option<&[T]>;

    fn lines(&self) -> lines::Lines<T, U>;
}

impl<T, U, G: Grid<T, U>> Crop<T, U, ops::RangeInclusive<Index<U>>> for G {
    fn crop(&self, range: ops::RangeInclusive<Index<U>>) -> Slice2D<T, U> {
        self.crop(*range.start()..*range.end() + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: Grid<T, U>> Crop<T, U, ops::RangeFrom<Index<U>>> for G {
    fn crop(&self, range: ops::RangeFrom<Index<U>>) -> Slice2D<T, U> {
        self.crop(range.start..Index::zero() + self.size())
    }
}

impl<T, U, G: Grid<T, U>> Crop<T, U, ops::RangeTo<Index<U>>> for G {
    fn crop(&self, range: ops::RangeTo<Index<U>>) -> Slice2D<T, U> {
        self.crop(Index::zero()..range.end)
    }
}

impl<T, U, G: Grid<T, U>> Crop<T, U, ops::RangeToInclusive<Index<U>>> for G {
    fn crop(&self, range: ops::RangeToInclusive<Index<U>>) -> Slice2D<T, U> {
        self.crop(Index::zero()..range.end + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: Grid<T, U>> Crop<T, U, ops::RangeFull> for G {
    fn crop(&self, _range: ops::RangeFull) -> Slice2D<T, U> {
        self.as_slice2d()
    }
}

impl<T, U, G: Grid<T, U>> Crop<T, U, euclid::TypedBox2D<usize, U>> for G {
    fn crop(&self, b: euclid::TypedBox2D<usize, U>) -> Slice2D<T, U> {
        self.crop(b.min..b.max)
    }
}

impl<T, U, G: Grid<T, U>> Crop<T, U, euclid::TypedRect<usize, U>> for G {
    fn crop(&self, rect: euclid::TypedRect<usize, U>) -> Slice2D<T, U> {
        self.crop(rect.origin..rect.bottom_right())
    }
}

pub trait GridMut<T, U>:
    Grid<T, U> + ops::IndexMut<Index<U>> + CropMut<T, U, ops::Range<Index<U>>>
{
    fn get_mut(&mut self, index: Index<U>) -> Option<&mut T>;

    fn as_slice2d_mut(&mut self) -> Slice2DMut<T, U>;

    fn line_mut(&mut self, y: usize) -> Option<&mut [T]>;
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, ops::RangeInclusive<Index<U>>> for G {
    fn crop_mut(&mut self, range: ops::RangeInclusive<Index<U>>) -> Slice2DMut<T, U> {
        self.crop_mut(*range.start()..*range.end() + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, ops::RangeFrom<Index<U>>> for G {
    fn crop_mut(&mut self, range: ops::RangeFrom<Index<U>>) -> Slice2DMut<T, U> {
        self.crop_mut(range.start..Index::zero() + self.size())
    }
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, ops::RangeTo<Index<U>>> for G {
    fn crop_mut(&mut self, range: ops::RangeTo<Index<U>>) -> Slice2DMut<T, U> {
        self.crop_mut(Index::zero()..range.end)
    }
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, ops::RangeToInclusive<Index<U>>> for G {
    fn crop_mut(&mut self, range: ops::RangeToInclusive<Index<U>>) -> Slice2DMut<T, U> {
        self.crop_mut(Index::zero()..range.end + euclid::TypedVector2D::new(1, 1))
    }
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, ops::RangeFull> for G {
    fn crop_mut(&mut self, _range: ops::RangeFull) -> Slice2DMut<T, U> {
        self.as_slice2d_mut()
    }
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, euclid::TypedBox2D<usize, U>> for G {
    fn crop_mut(&mut self, b: euclid::TypedBox2D<usize, U>) -> Slice2DMut<T, U> {
        self.crop_mut(b.min..b.max)
    }
}

impl<T, U, G: GridMut<T, U>> CropMut<T, U, euclid::TypedRect<usize, U>> for G {
    fn crop_mut(&mut self, rect: euclid::TypedRect<usize, U>) -> Slice2DMut<T, U> {
        self.crop_mut(rect.origin..rect.bottom_right())
    }
}