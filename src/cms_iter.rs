use std::fmt::Debug;
use std::ops::{AddAssign, SubAssign, Add, DivAssign};
use std::hash::Hash;
use num_traits::Bounded;

use crate::cms::CountMinSketch;

/// An iterator struct to convert `CountMinSketch` instance into 
/// so that the matrix can be iterated over.
/// ```
/// use decentrust::cms::CountMinSketch;
/// 
/// let mut cms = &CountMinSketch::<u64>::default();
/// let mut iter = cms.into_iter();
/// assert_eq!(Some(&u64::default()), iter.next());
/// ```
pub struct CountMinSketchIter<'a, T>
where 
    T: AddAssign 
    + SubAssign 
    + DivAssign 
    + Add<Output = T>
    + Hash
    + Ord 
    + Debug 
    + Bounded
{
    pub cms: &'a CountMinSketch<T>,
    pub row: usize,
    pub col: usize,
}


/// An iterator struct to convert owned `CountMinSketch` into
/// ```
/// use decentrust::cms::CountMinSketch;
/// let mut cms = CountMinSketch::<u64>::default();
/// let mut iter = cms.into_iter();
/// assert_eq!(Some(u64::default()), iter.next());
/// ```
pub struct CountMinSketchIntoIter<T>
where 
    T: AddAssign 
    + SubAssign 
    + DivAssign
    + Hash 
    + Ord 
    + Debug 
    + Bounded
{
    pub matrix: Vec<Vec<T>>,
    pub row: usize,
    pub col: usize,
}


/// Implements the `next` method on CountMinSketchIter
impl<'a, T> Iterator for CountMinSketchIter<'a, T> 
where
    T: AddAssign 
    + SubAssign 
    + DivAssign 
    + Add<Output = T>
    + Hash 
    + Ord 
    + Debug 
    + Bounded
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.cms.depth {
            return None;
        }

        let element = &self.cms.matrix[self.row][self.col];

        self.col += 1;
        if self.col >= self.cms.width {
            self.col = 0;
            self.row += 1;
        }

        Some(element)
    }
}

/// Converts a borrowed `CountMinSketch` instance into 
/// a type that implements iterator
impl<'a, T> IntoIterator for &'a CountMinSketch<T>
where
    T: AddAssign 
    + SubAssign 
    + DivAssign 
    + Add<Output = T>
    + Hash 
    + Ord 
    + Debug 
    + Bounded
{
    type Item = &'a T;
    type IntoIter = CountMinSketchIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CountMinSketchIter {
            cms: self,
            row: 0,
            col: 0,
        }
    }
}

/// Implements the `next` method on CountMinSketchIntoIter type
impl<T> Iterator for CountMinSketchIntoIter<T> 
where 
    T: AddAssign 
    + SubAssign 
    + DivAssign 
    + Clone 
    + Hash 
    + Ord
    + Debug 
    + Bounded
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.matrix.len() {
            return None;
        }

        let element = self.matrix[self.row][self.col].clone();

        self.col +=1;

        if self.col >= self.matrix[self.row].len() {
            self.col = 0;
            self.row += 1;
        }

        Some(element)
    }
}


/// Converts an owned `CountMinSketch` instance into a type that 
/// implements `Iterator`
impl<T> IntoIterator for CountMinSketch<T>
where
    T: AddAssign 
    + SubAssign 
    + DivAssign 
    + Add<Output = T> 
    + Clone
    + Hash 
    + Ord 
    + Debug 
    + Bounded
{
    type Item = T;
    type IntoIter = CountMinSketchIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CountMinSketchIntoIter {
            matrix: self.matrix.clone(),
            row: 0,
            col: 0,
        }
    }
}
