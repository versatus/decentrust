use std::hash::Hash;
use std::ops::{AddAssign, DivAssign, SubAssign, Add, Mul, Div, Sub};
use num_traits::Bounded;
use std::marker::PhantomData;

use crate::cms::CountMinSketch;
use crate::honest_peer::HonestPeer;

/// A struct to track local and global trust of peers in a 
/// peer to peer data sharing network. Trust scores 
/// can be incremented or decremented, when the node holding this 
/// struct witnesses trustworthy or malicious behaviours by a peer 
/// respectively. 
///
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hash;
/// use std::ops::{AddAssign, SubAssign, DivAssign, Add, Mul, Div, Sub};
/// use std::marker::PhantomData;
/// use decentrust::cms::CountMinSketch;
/// use num_traits::Bounded;
///
/// pub struct LightHonestPeer<K, V> 
/// where 
///     K: Eq + Hash + Clone,
///     V: AddAssign
///     + DivAssign
///     + SubAssign 
///     + Add<Output = V> 
///     + Mul<Output = V> 
///     + Div<Output = V> 
///     + Sub<Output = V> 
///     + Copy 
///     + Default 
///     + Bounded
///     + Ord 
///     + Hash
///
/// {
///     local_trust: CountMinSketch<V>,
///     global_trust: CountMinSketch<V>,
///     normalized_local_trust: CountMinSketch<V>,
///     normalized_global_trust: CountMinSketch<V>,
///     id: Option<PhantomData<K>>
/// }
/// ```
pub struct LightHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign 
    + DivAssign 
    + SubAssign 
    + Add<Output = V> 
    + Mul<Output = V> 
    + Div<Output = V> 
    + Sub<Output = V> 
    + Copy 
    + Default 
    + Bounded
    + Ord 
    + Hash
{
    local_trust: CountMinSketch<V>,
    global_trust: CountMinSketch<V>,
    normalized_local_trust: CountMinSketch<V>,
    normalized_global_trust: CountMinSketch<V>,
    pub id_type: Option<PhantomData<K>>
}


impl<K, V> LightHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign 
    + DivAssign 
    + SubAssign 
    + Add<Output = V> 
    + Mul<Output = V> 
    + Div<Output = V> 
    + Sub<Output = V> 
    + Copy 
    + Default 
    + Bounded 
    + Ord 
    + Hash
{
    /// Creates a new `LightHonestPeer` struct with no peers in it.
    /// 
    /// ```
    /// use decentrust::probabilistic::LightHonestPeer;
    /// use decentrust::honest_peer::HonestPeer;
    /// use ordered_float::OrderedFloat;
    ///
    /// let mut hp = LightHonestPeer::<String, OrderedFloat<f64>>::new();
    /// 
    /// assert_eq!(0, hp.local_raw_len());
    /// assert_eq!(0, hp.global_raw_len());
    /// ```
    pub fn new() -> Self {
        LightHonestPeer { 
            local_trust: CountMinSketch::<V>::default(), 
            global_trust: CountMinSketch::<V>::default(),
            normalized_local_trust: CountMinSketch::<V>::default(),
            normalized_global_trust: CountMinSketch::<V>::default(),
            id_type: None,
        }
    }

    /// Creates a new `LightHonestPeer` instance from a given 
    /// `CountMinSketch` error bound, an overestimation probability,
    /// and the maximum expected number of entries.
    /// ```
    /// use decentrust::probabilistic::LightHonestPeer;
    /// use decentrust::honest_peer::HonestPeer;
    /// use ordered_float::OrderedFloat;
    /// use num_traits::Bounded;
    ///
    /// let mut hp = LightHonestPeer::<String, OrderedFloat<f64>>::new_from_bounds(
    ///     50f64,
    ///     0.0001f64,
    ///     3000f64,
    ///     OrderedFloat::<f64>::min_value(),
    ///     OrderedFloat::<f64>::max_value()
    /// );
    /// 
    /// assert_eq!(0, hp.local_raw_len());
    /// assert_eq!(0, hp.global_raw_len());
    /// ```
    pub fn new_from_bounds(
        error_bound: f64, 
        probability: f64, 
        max_entries: f64,
        min: V,
        max: V 
    ) -> Self {
        let sketch = CountMinSketch::new_from_bounds(
            error_bound, 
            probability, 
            max_entries, 
            min, 
            max
        );

        LightHonestPeer {
            local_trust: sketch.clone(),
            global_trust: sketch.clone(),
            normalized_local_trust: sketch.clone(),
            normalized_global_trust: sketch.clone(),
            id_type: None
        }
    }
}

impl<K, V> HonestPeer for LightHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign 
        + DivAssign 
        + SubAssign 
        + Add<Output = V> 
        + Mul<Output = V> 
        + Div<Output = V> 
        + Sub<Output = V> 
        + Copy 
        + Default 
        + Bounded 
        + Ord 
        + Hash
{
    type Map = CountMinSketch<V>;
    type Key = K;
    type Value = V;

    /// Initalizes a local trust value for a newly discovered peer
    fn init_local(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.local_trust.increment(key, init_value);
        self.normalize_local();
    }

    /// Updates a local trust value for a given peer
    fn update_local(&mut self, key: &Self::Key, trust_delta: Self::Value) {
        self.local_trust.increment(key, trust_delta);
        self.normalize_local();
    }

    /// returns the raw (unnormalized) estimate for a given peer
    fn get_raw_local(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.local_trust.estimate(key))
    }

    /// returns the normalized estimate for a given peer
    fn get_normalized_local(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.normalized_local_trust.estimate(key))
    }

    /// initializes a global trust value for a newly discovered peer
    fn init_global(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.global_trust.increment(key, init_value);
        self.normalize_global();
    }

    /// updates a global trust value for a given peer
    fn update_global(&mut self, key: &Self::Key, trust_delta: Self::Value) {
        self.global_trust.increment(key, trust_delta);
        self.normalize_global();
    }

    /// returns the raw (unnormalized) estimate for a given peer
    fn get_raw_global(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.global_trust.estimate(key))
    }

    /// returns the normalized estimate for a given peer
    fn get_normalized_global(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.normalized_global_trust.estimate(key))
    }

    /// returns the raw (unnormalized) local `CountMinSketch` for the 
    /// current instance
    fn get_raw_local_map(&self) -> Self::Map {
        self.local_trust.clone()
    }

    /// returns the normalized local `CountMinSketch` for the current 
    /// instance
    fn get_normalized_local_map(&self) -> Self::Map {
        self.normalized_local_trust.clone()
    }

    /// returns the raw global `CountMinSketch` for the current instance
    fn get_raw_global_map(&self) -> Self::Map {
        self.global_trust.clone()
    }

    /// returns the normalized global `CountMinSketch` for the current 
    /// instance
    fn get_normalized_global_map(&self) -> Self::Map {
        self.normalized_global_trust.clone()
    }

    /// normalizes the local trust matrix
    fn normalize_local(&mut self) {
        self.normalized_local_trust.matrix = self.local_trust.normalize_estimates();
    }

    /// normalizes the global trust matrix
    fn normalize_global(&mut self) {
        self.normalized_global_trust.matrix = self.global_trust.normalize_estimates();
    }

    /// returns the number of non-default entries in the raw local 
    /// `CountMinSketch`(currently broken)
    fn local_raw_len(&self) -> usize {
        self.local_trust.get_estimate_length()
    }

    /// returns the number of non-default entries in the normalized local 
    /// `CountMinSketch` (currently broken)
    fn local_normalized_len(&self) -> usize {
        self.normalized_local_trust.get_estimate_length()
    }

    /// returns the number of non-default entries in the global raw 
    /// `CountMinSketch` (currently broken)
    fn global_raw_len(&self) -> usize {
        self.global_trust.get_estimate_length()
    }

    /// returns the number of non-default entries in the global 
    /// normalized `CountMinSketch` (currently broken)
    fn global_normalized_len(&self) -> usize {
        self.normalized_global_trust.get_estimate_length()
    }
}
