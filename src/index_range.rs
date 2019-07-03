use crate::types::{Index, Size};
use std::ops;

pub trait IndexRange<U> {
    fn normalize(self, size: Size<U>) -> ops::Range<Index<U>>;
}

impl<U> IndexRange<U> for ops::Range<Index<U>> {
    fn normalize(self, _size: Size<U>) -> Self {
        self
    }
}

impl<U> IndexRange<U> for ops::RangeInclusive<Index<U>> {
    fn normalize(self, _size: Size<U>) -> ops::Range<Index<U>> {
        *self.start()..*self.end() + euclid::TypedVector2D::new(1, 1)
    }
}

impl<U> IndexRange<U> for ops::RangeFrom<Index<U>> {
    fn normalize(self, size: Size<U>) -> ops::Range<Index<U>> {
        self.start..Index::zero() + size
    }
}

impl<U> IndexRange<U> for ops::RangeTo<Index<U>> {
    fn normalize(self, _size: Size<U>) -> ops::Range<Index<U>> {
        Index::zero()..self.end
    }
}

impl<U> IndexRange<U> for ops::RangeToInclusive<Index<U>> {
    fn normalize(self, size: Size<U>) -> ops::Range<Index<U>> {
        (Index::zero()..=self.end).normalize(size)
    }
}

impl<U> IndexRange<U> for ops::RangeFull {
    fn normalize(self, size: Size<U>) -> ops::Range<Index<U>> {
        Index::zero()..Index::zero() + size
    }
}