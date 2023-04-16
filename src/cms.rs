#![allow(unused)]
use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::ops::{AddAssign, SubAssign, Add, DivAssign};
use siphasher::sip::SipHasher13;
use std::num::Wrapping;
use std::f64::consts::E;
use num_traits::Bounded;
use std::default::Default;

/// CountMinSketch is a probabilistic data structure for estimating 
/// values, typically frequencies in a data stream. In this crate 
/// it is designed to be a proabilistic reputation tracking structure
/// but can be used for tracking frequencies in a data stream as well 
/// as other use cases for probabilistic data structures where small 
/// overestimations within a given error bound and with a given 
/// proability is acceptable, but underestimations are never acceptable
/// ```
/// use std::collections::hash_map::RandomState;
/// use std::ops::{AddAssign, SubAssign, DivAssign, Add};
/// use std::hash::Hash;
///
/// #[derive(Clone, Debug)]
/// pub struct CountMinSketch<T>
/// where
///     T: AddAssign + SubAssign + DivAssign + Add<Output = T> + Ord + Hash
/// {
///     pub width: usize,
///     pub depth: usize,
///     pub matrix: Vec<Vec<T>>,
///     hash_builder: RandomState,
///     max: T,
///     min: T,
/// }
/// ```
#[derive(Clone, Debug)]
pub struct CountMinSketch<T> 
where
    T: AddAssign 
    + SubAssign 
    + DivAssign
    + Add<Output = T>
    + Ord 
    + Hash
{
    pub width: usize,
    pub depth: usize,
    pub matrix: Vec<Vec<T>>,
    hash_builder: RandomState,
    max: T,
    min: T 
}

impl<T> CountMinSketch<T> 
where
    T: AddAssign 
    + SubAssign 
    + DivAssign
    + Add<Output = T> 
    + Default 
    + Copy 
    + Bounded
    + Ord 
    + Hash
{
    /// Creates a new CountMinSketch struct with a width,
    /// depth, min value and max value
    ///
    /// ```
    /// use decentrust::cms::CountMinSketch;
    /// use ordered_float::OrderedFloat;
    ///
    /// let cms = CountMinSketch::<OrderedFloat<f64>>::new(
    /// 3000, 10, 0f64.into(), 1000f64.into());
    ///
    /// println!("{:?}", cms);
    /// ```
    pub fn new(width: usize, depth: usize, min: T, max: T) -> Self {
        let matrix = vec![vec![T::default(); width]; depth];
        let hash_builder = RandomState::new();

        CountMinSketch {
            width,
            depth,
            matrix,
            hash_builder,
            max,
            min,
        }
    }

    /// Creates a new CountMinSketch from desired bounds and 
    /// probability of overestimation, and the maximum number 
    /// of expected entries.
    ///
    /// ```
    /// use decentrust::cms::CountMinSketch;
    /// use ordered_float::OrderedFloat;
    ///
    /// let cms = CountMinSketch::<OrderedFloat<f64>>::new_from_bounds(
    ///     50f64, 
    ///     0.001, 
    ///     10000f64, 
    ///     0f64.into(), 
    ///     1000f64.into()
    /// );
    ///
    /// println!("{:?}", cms);
    /// ```
    pub fn new_from_bounds(
        error_bound: f64, 
        probability: f64, 
        max_entries: f64,
        min: T,
        max: T 
    ) -> Self {

        let (width, depth) = {
            CountMinSketch::<T>::calculate_width_and_depth(
                error_bound, probability, max_entries
            ) 
        };

        CountMinSketch::new(width, depth, min, max)
    }

    /// Takes a reference to an item implementing the `Hash` trait
    /// and a index representing the hash function. It creates a new
    /// hasher using the hash_builder, hashes the item, and returns
    /// the hashed value modulo the width of the sketch matric.
    fn hash_pair(&self, item: &impl Hash, index: usize) -> usize {
        let mut hasher = self.hash_builder.build_hasher();
        let wrapping_index = Wrapping(index as u64);
        let wrapping_hash = Wrapping(hasher.finish());

        (wrapping_hash + wrapping_index).0 as usize % self.width
    }

    /// Takes a reference to an item that implements `Hash` and
    /// returns a vector hashed values for each hash function
    /// (one value for each row in the sketch matrix)
    fn hash_functions(&self, item: &impl Hash) -> Vec<usize> {
        (0..self.depth).map(|i| self.hash_pair(item, i)).collect()
    }

    /// Takes a reference to an item implementing `Hash` and 
    /// a value of type `T` to be added. It calculates the hashes
    /// for the item using `hash_functions` method, and updates
    /// the sketch matrix adding the given value at respective
    /// positions
    ///
    /// # Examples
    ///
    /// ```
    /// // Need some float primitives to calculate 
    /// // depth and width based on desired error_bound and probability
    /// use decentrust::cms::CountMinSketch;
    /// 
    /// // Create CountMinSketch with calculated depth and width
    /// let mut cms = CountMinSketch::<i64>::default();
    ///
    /// // Create a mock node_id let node_id = "node_1";
    /// let node_id = "node1";
    ///
    /// // Increment the node's reputation score;
    /// cms.increment(&node_id, 100);
    /// let estimated_score = cms.estimate(&node_id);
    /// println!("Estimated reputation score after increment: {}", estimated_score);
    ///
    /// // Decrement the reputation score
    /// cms.increment(&node_id, -50);
    /// let estmimated_score = cms.estimate(&node_id);
    ///
    /// println!("Estimated reputation score after decrement: {}", estimated_score);
    ///
    /// ```
    pub fn increment(&mut self, item: &impl Hash, value: T) {
        let hashes = self.hash_functions(item);
        (0..self.depth).into_iter()
            .for_each(|i| {
                self.matrix[i][hashes[i]] += value;
            }
        )
    }

    /// Takes a reference to an item implementing `Hash` and
    /// returns an estimate of the value for that item. It calculates
    /// the hash values for the item using hash_functions and returns 
    /// the minimum value found at the respective positions in the sketch
    /// matrix.
    ///
    /// ```
    /// use decentrust::cms::CountMinSketch;
    /// 
    /// // Create CountMinSketch with calculated depth and width
    /// let mut cms = CountMinSketch::<i64>::default();
    ///
    /// // Create a mock node_id let node_id = "node_1";
    /// let node_id = "node1";
    ///
    /// // Increment the node's reputation score;
    /// cms.increment(&node_id, 10);
    /// let estimated_score = cms.estimate(&node_id);
    /// assert_eq!(estimated_score, 10);
    /// ```
    pub fn estimate(&self, item: &impl Hash) -> T {
        let hashes = self.hash_functions(item);
        let mut min_estimate = self.matrix[0][hashes[0]];
        (1..self.depth).into_iter().for_each(|i| {
            min_estimate = std::cmp::min(min_estimate, self.matrix[i][hashes[i]]); 
        });

        min_estimate
    }

    /// Helper method to calculate width and depth of a CountMinSketch 
    /// internally. Used in the `new_from_bounds` initializer method
    fn calculate_width_and_depth(
        error_bound: f64, 
        probability: f64, 
        max_entries: f64
    ) -> (usize, usize) { 
        let width = f64::ceil(1f64 / (error_bound / max_entries)) as usize;
        let depth = f64::ceil(f64::ln(probability)) as usize;

        (width, depth)
    }

    pub fn get_min(&self) -> T {
        self.min
    }

    pub fn get_max(&self) -> T {
        self.max
    }

    /// Loops through the entire matrix and extracts summed value 
    /// from each row. It then loops through every row and column 
    /// in the matrix and divides each value by the summed value for 
    /// the given row to create a normalized value. Currently this 
    /// should only return a value between 0 or 1, i.e. a float.
    /// ```
    /// use decentrust::cms::CountMinSketch;
    /// use ordered_float::OrderedFloat;
    /// 
    /// // Create CountMinSketch with calculated depth and width
    /// let mut cms = CountMinSketch::<OrderedFloat<f64>>::default();
    ///
    /// // Create a mock node_id let node_id = "node_1";
    /// let node_id = "node1";
    ///
    /// // Increment the node's reputation score;
    /// cms.increment(&node_id, 10f64.into());
    /// let estimated_score = cms.estimate(&node_id);
    /// assert_eq!(estimated_score, OrderedFloat(10f64));
    ///
    /// let mut normalized_cms = cms.clone();
    /// normalized_cms.matrix = cms.normalize_estimates(); 
    ///
    /// let normalized_estimate = normalized_cms.estimate(&node_id);
    /// println!("{}", normalized_estimate);
    /// 
    /// ```
    // TODO: add normalization factor to be able to return integers 
    // both signed and unsigned, by multiplying the float value for 
    // each normalized value by the factor. Factors should be orders 
    // of decimal magnitude, i.e. should always be modulo 10 == 0.
    //
    pub fn normalize_estimates(&self) -> Vec<Vec<T>> {
        let mut total_vec: Vec<T> = vec![T::default(); self.depth]; 
        let mut new_matrix = vec![vec![T::default(); self.width]; self.depth];
        for (idx, row) in self.matrix.iter().enumerate() {
            let row_acc = row.iter().fold(T::default(), |acc, v| {
                acc + *v
            });

            total_vec[idx] = row_acc;

        }

        new_matrix.iter_mut()
            .enumerate()
            .for_each(|(idx, row)| {
                let total_trust = total_vec[idx];
                row.iter_mut().for_each(|v| {
                    *v /= total_trust;
                });
            });

        new_matrix
    }

    /// Returns the length of all non-default entries in the 
    /// `CountMinSketch` instance to get a probabilistic length 
    /// of the number of items the instance is tracking.
    ///
    /// ```
    /// use decentrust::cms::CountMinSketch;
    /// use ordered_float::OrderedFloat;
    /// 
    /// // Create CountMinSketch with calculated depth and width
    /// let mut cms = CountMinSketch::<OrderedFloat<f64>>::default();
    ///
    /// // Create a mock node_id let node_id = "node_1";
    /// let node_id = "node1";
    ///
    /// // Increment the node's reputation score;
    /// cms.increment(&node_id, 10f64.into());
    /// let len = cms.get_estimate_length();
    ///
    /// println!("{}", len);
    /// ```
    ///
    pub fn get_estimate_length(&self) -> usize {
        let len = self.matrix
            .iter()
            .fold(0usize, |acc, row| {
                let non_default_count = row.iter()
                    .filter(|&v| *v != T::default())
                    .count();

                let weighted_estimate = 
                    non_default_count / self.depth;

                acc + weighted_estimate
            });

        len
    }
}

/// Implements the default trait for count_min_sketch for a 
/// given T value. 
impl<T> Default for CountMinSketch<T> 
where
    T: AddAssign 
    + SubAssign 
    + DivAssign
    + Add<Output = T> 
    + Hash 
    + Default 
    + Copy 
    + Ord 
    + Bounded
    
{
    fn default() -> Self {
        Self::new(3000, 10, T::min_value(), T::max_value())
    }
}
