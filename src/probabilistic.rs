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
/// use std::ops::{AddAssign, DivAssign, Add, Mul};
/// use std::marker::PhantomData,
///
/// pub struct LightHonestPeer<K, V> 
/// where 
///     K: Eq + Hash + Cline
///     V: AddAssign + DivAssign + Add<Output = V> + Mul<Output = V> + Copy + Default
/// {
///     local_trust: CountMinSketch<V>,
///     global_trust: CountMinSketch<V>
///     normalized_local_trust: CountMinSketch<V>,
///     normalized_global_trust: CountMinSketch<V>
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
    + Hash 
    + Ord
{
    /// Creates a new `LightHonestPeer` struct with no peers in it.
    /// 
    /// ```
    /// use decentrust::honest_peer::LightHonestPeer;
    ///
    /// let mut hp: LightHonestPeer<String, f64> = LightHonestPeer::new();
    /// 
    /// assert_eq!(0, hp.local_trust_len());
    /// assert_eq!(0, hp.global_trust_len());
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

    pub fn new_with_bounds(
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
        + Hash 
        + Ord,
{
    type Map = CountMinSketch<V>;
    type Key = K;
    type Value = V;

    fn init_local(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.local_trust.increment(key, init_value);
        self.normalize_local();
    }

    fn update_local(&mut self, key: &Self::Key, trust_delta: Self::Value) {
        self.local_trust.increment(key, trust_delta);
        self.normalize_local();
    }

    fn get_raw_local(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.local_trust.estimate(key))
    }

    fn get_normalized_local(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.normalized_local_trust.estimate(key))
    }

    fn init_global(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.global_trust.increment(key, init_value);
        self.normalize_global();
    }

    fn update_global(&mut self, key: &Self::Key, trust_delta: Self::Value) {
        self.global_trust.increment(key, trust_delta);
        self.normalize_global();
    }

    fn get_raw_global(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.global_trust.estimate(key))
    }

    fn get_normalized_global(&self, key: &Self::Key) -> Option<Self::Value> {
        Some(self.normalized_global_trust.estimate(key))
    }

    fn get_raw_local_map(&self) -> Self::Map {
        self.local_trust.clone()
    }

    fn get_normalized_local_map(&self) -> Self::Map {
        self.normalized_local_trust.clone()
    }

    fn get_raw_global_map(&self) -> Self::Map {
        self.global_trust.clone()
    }

    fn get_normalized_global_map(&self) -> Self::Map {
        self.normalized_global_trust.clone()
    }

    fn normalize_local(&mut self) {
        self.normalized_local_trust.matrix = self.local_trust.normalize_estimates();
    }

    fn normalize_global(&mut self) {
        self.normalized_global_trust.matrix = self.global_trust.normalize_estimates();
    }

    fn local_raw_len(&self) -> usize {
        self.local_trust.get_estimate_length()
    }

    fn local_normalized_len(&self) -> usize {
        self.normalized_local_trust.get_estimate_length()
    }

    fn global_raw_len(&self) -> usize {
        self.global_trust.get_estimate_length()
    }

    fn global_normalized_len(&self) -> usize {
        self.normalized_global_trust.get_estimate_length()
    }
}
