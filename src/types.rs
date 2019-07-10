use euclid;

pub type Size<U = euclid::UnknownUnit> = euclid::TypedSize2D<usize, U>;

pub type Index<U = euclid::UnknownUnit> = euclid::TypedPoint2D<usize, U>;

pub type IndexOffset<U = euclid::UnknownUnit> = euclid::TypedVector2D<isize, U>;