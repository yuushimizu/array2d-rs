use std::ops;

pub type Size<U> = euclid::TypedSize2D<usize, U>;

pub type Index<U> = euclid::TypedPoint2D<usize, U>;

pub type IndexOffset<U> = euclid::TypedVector2D<isize, U>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array2D<T, U> {
    items: Vec<T>,
    size: Size<U>,
}

impl<T, U> Array2D<T, U> {
    /// # Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(10, 10), "neko".to_string());
    /// assert_eq!("neko".to_string(), array2d[Index::new(4, 2)]);
    /// ```
    pub fn new(size: Size<U>, initial_value: T) -> Self
    where
        T: Clone,
    {
        Self {
            items: vec![initial_value; size.area()],
            size,
        }
    }

    /// # Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::from_constructor(Size::new(5, 5), |i| i.x * 10 + i.y);
    /// assert_eq!(23, array2d[Index::new(2, 3)]);
    /// ```
    pub fn from_constructor(size: Size<U>, f: impl FnMut(Index<U>) -> T) -> Self {
        use euclid_ext::Points;
        Self {
            items: euclid::TypedRect::from_size(size)
                .points()
                .map(f)
                .collect::<Vec<_>>(),
            size,
        }
    }

    fn index_at_unchecked(&self, index: Index<U>) -> usize {
        index.y * self.size.width + index.x
    }

    fn in_bounds(&self, index: Index<U>) -> bool {
        euclid::TypedRect::new(Index::zero(), self.size).contains(&index)
    }

    fn index_at(&self, index: Index<U>) -> Option<usize> {
        if self.in_bounds(index) {
            Some(self.index_at_unchecked(index))
        } else {
            None
        }
    }

    fn debug_asserted_index_at(&self, index: Index<U>) -> usize {
        debug_assert!(
            self.in_bounds(index),
            "index out of bounds: index = {}, size = {}",
            index,
            self.size
        );
        self.index_at_unchecked(index)
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(50, 100), 0);
    /// assert_eq!(Size::new(50, 100), array2d.size());
    /// ```
    pub fn size(&self) -> Size<U> {
        self.size
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(48, 32), 1);
    /// assert_eq!(Index::new(0, 0)..Index::new(48, 32), array2d.index_range());
    /// ```
    pub fn index_range(&self) -> ops::Range<Index<U>> {
        Index::zero()..Index::zero() + self.size
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(3, 4), 10);
    /// assert_eq!(
    ///     vec![
    ///         Index::new(0, 0), Index::new(1, 0), Index::new(2, 0),
    ///         Index::new(0, 1), Index::new(1, 1), Index::new(2, 1),
    ///         Index::new(0, 2), Index::new(1, 2), Index::new(2, 2),
    ///         Index::new(0, 3), Index::new(1, 3), Index::new(2, 3)
    ///     ],
    ///     array2d.indices().collect::<Vec<_>>());
    /// ```
    pub fn indices(&self) -> impl Iterator<Item = Index<U>> {
        use euclid_ext::Points;
        self.index_range().points()
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::from_constructor(Size::new(5, 8), |i| i.x * 10 + i.y);
    /// assert_eq!(Some(&36), array2d.get(Index::new(3, 6)));
    /// ```
    pub fn get(&self, index: Index<U>) -> Option<&T> {
        self.index_at(index).map(|index| &self.items[index])
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let mut array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(10, 10), 0);
    /// *array2d.get_mut(Index::new(3, 5)).unwrap() = 100;
    /// assert_eq!(100, array2d[Index::new(3, 5)]);
    /// ```
    pub fn get_mut(&mut self, index: Index<U>) -> Option<&mut T> {
        self.index_at(index)
            .map(move |index| &mut self.items[index])
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::from_constructor(Size::new(4, 5), |i| i.x * 10 + i.y);
    /// assert_eq!(Some(vec![2, 12, 22, 32].as_ref()), array2d.line(2));
    /// ```
    pub fn line(&self, y: usize) -> Option<&[T]> {
        self.index_at(Index::new(0, y))
            .map(|start| &self.items[start..start + self.size.width])
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let mut array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(3, 4), 0);
    /// let line = array2d.line_mut(1).unwrap();
    /// line[2] = 100;
    /// assert_eq!(100, array2d[Index::new(2, 1)]);
    /// ```
    pub fn line_mut(&mut self, y: usize) -> Option<&mut [T]> {
        self.index_at(Index::new(0, y))
            .map(move |start| &mut self.items[start..start + self.size.width])
    }

    /// Examples
    /// ```
    /// # use array2d::{Array2D, Size, Index};
    /// let array2d = Array2D::<_, euclid::UnknownUnit>::from_constructor(Size::new(4, 6), |i| i.x * 10 + i.y);
    /// assert_eq!(
    ///     vec![
    ///         vec![0, 10, 20, 30],
    ///         vec![1, 11, 21, 31],
    ///         vec![2, 12, 22, 32],
    ///         vec![3, 13, 23, 33],
    ///         vec![4, 14, 24, 34],
    ///         vec![5, 15, 25, 35]
    ///     ],
    ///     array2d.lines().map(|line| line.to_vec()).collect::<Vec<_>>());
    /// ```
    pub fn lines<'a>(&'a self) -> impl Iterator<Item = &[T]> + 'a {
        (0..self.size.height).map(move |y| self.line(y).unwrap())
    }
}

/// Examples
/// ```
/// # use array2d::{Array2D, Size, Index};
/// let array2d = Array2D::<_, euclid::UnknownUnit>::from_constructor(Size::new(8, 8), |i| i.x * 10 + i.y);
/// assert_eq!(52, array2d[Index::new(5, 2)]);
/// ```
impl<T, U> ops::Index<Index<U>> for Array2D<T, U> {
    type Output = T;

    fn index(&self, index: Index<U>) -> &Self::Output {
        self.items.index(self.debug_asserted_index_at(index))
    }
}

/// Examples
/// ```
/// # use array2d::{Array2D, Size, Index};
/// let mut array2d = Array2D::<_, euclid::UnknownUnit>::new(Size::new(10, 10), 0);
/// array2d[Index::new(3, 8)] = 100;
/// assert_eq!(100, array2d[Index::new(3, 8)]);
/// ```
impl<T, U> ops::IndexMut<Index<U>> for Array2D<T, U> {
    fn index_mut(&mut self, index: Index<U>) -> &mut Self::Output {
        let index = self.debug_asserted_index_at(index);
        self.items.index_mut(index)
    }
}
