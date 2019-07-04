pub trait AsSlice {
    type Item;

    fn as_slice(&self) -> &[Self::Item];
}

impl<T> AsSlice for &[T] {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> AsSlice for &mut [T] {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

pub trait AsMutSlice: AsSlice {
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<T> AsMutSlice for &mut [T] {
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}

impl<T> AsSlice for Vec<T> {
    type Item = T;

    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> AsMutSlice for Vec<T> {
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self
    }
}