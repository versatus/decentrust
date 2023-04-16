use std::ops::{AddAssign, SubAssign, Add, DivAssign};
use crate::cms::CountMinSketch;


pub struct CountMinSketchIter<'a, T>
where 
    T: AddAssign + SubAssign + DivAssign + Add<Output = T>,
{
    pub cms: &'a CountMinSketch<T>,
    pub row: usize,
    pub col: usize,
}

pub struct CountMinSketchIntoIter<T>
where 
    T: AddAssign + SubAssign + DivAssign,
{
    pub matrix: Vec<Vec<T>>,
    pub row: usize,
    pub col: usize,
}

impl<'a, T> Iterator for CountMinSketchIter<'a, T> 
where
    T: AddAssign + SubAssign + DivAssign + Add<Output = T>,
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

impl<'a, T> IntoIterator for &'a CountMinSketch<T>
where
    T: AddAssign + SubAssign + DivAssign + Add<Output = T>,
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

impl<T> Iterator for CountMinSketchIntoIter<T> 
where 
    T: AddAssign + SubAssign + DivAssign + Clone, 
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


impl<T> IntoIterator for CountMinSketch<T>
where
    T: AddAssign + SubAssign + DivAssign + Add<Output = T> + Clone,
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
