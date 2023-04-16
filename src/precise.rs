use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{AddAssign, DivAssign, SubAssign, Add, Mul, Div, Sub};
use num_traits::Bounded;

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
///
/// pub struct PreciseHonestPeer<K, V> 
/// where 
///     K: Eq + Hash + Clone,
///     V: AddAssign + DivAssign + Add<Output = V> + Mul<Output = V> + Copy + Default
/// {
///     local_trust: HashMap<K, V>,
///     global_trust: HashMap<K, V>
///     normalized_local_trust: HashMap<K, V>,
///     normalized_global_trust: HashMap<K, V>,
/// }
/// ```
pub struct PreciseHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign + DivAssign + Add<Output = V> + Mul<Output = V> + Div<Output = V> + Sub<Output = V> + Copy + Default
{
    local_trust: HashMap<K, V>,
    global_trust: HashMap<K, V>,
    normalized_local_trust: HashMap<K, V>,
    normalized_global_trust: HashMap<K, V>
}


impl<K, V> PreciseHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone,
    V: AddAssign + DivAssign + SubAssign + Add<Output = V> + Mul<Output = V> + Div<Output = V> + Sub<Output = V> + Copy + Default
{
    /// Creates a new `PreciseHonestPeer` struct with no peers in it.
    /// 
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    ///
    /// let mut hp: PreciseHonestPeer<String, f64> = PreciseHonestPeer::new();
    /// 
    /// assert_eq!(0, hp.local_trust_len());
    /// assert_eq!(0, hp.global_trust_len());
    /// ```
    pub fn new() -> Self {
        PreciseHonestPeer { 
            local_trust: HashMap::new(), 
            global_trust: HashMap::new(),
            normalized_local_trust: HashMap::new(),
            normalized_global_trust: HashMap::new(),
        }
    }
}


impl<K, V> HonestPeer for PreciseHonestPeer<K, V> 
where 
    K: Eq + std::hash::Hash + Clone,  
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
    type Map = HashMap<K, V>;
    type Key =  K;
    type Value = V; 

    /// Initialize the local trust score of a newly discovered peer 
    ///
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// 
    /// let mut hp: PreciseHonestPeer<String, f64> = PreciseHonestPeer::new();
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_local_trust("node1".to_string(), 0.01);
    ///
    /// assert_eq!(hp.local_raw_len(), 1);
    /// assert_eq!(hp.local_normalized_len(), 1);
    ///
    /// ```
    fn init_local(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.local_trust.insert(key.clone(), init_value);
        self.normalize_local();
    }

    /// Updates the local trust score of a peer, and normalizes 
    /// the trust score map.
    ///
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    ///
    /// fn equal_floats(a: &f64, b: f64, epsilon: f64) -> bool {
    ///     (*a - b).abs() < epsilon
    /// }
    ///
    /// let mut hp: PreciseHonestPeer<String, f64> = PreciseHonestPeer::new();
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_local("node1".to_string(), 0.01);
    /// hp.init_local("node2".to_string(), 0.01);
    ///
    /// hp.update_local(&"node1".to_string(), 0.05);
    ///
    /// let local_total_trust = 0.01 + 0.01 + 0.05;
    /// let node_1_local_trust = 0.06 / local_total_trust;
    /// let node_2_local_trust = 0.01 / local_total_trust;
    ///
    /// // Since the init calls are the first in this instance,
    /// // normalization will necessarily mean that they equal 100% of the 
    /// // weight of the scores.
    /// if let Some(val) = hp.get_normalized_local(&"node1".to_string()) {
    ///     assert!(equal_floats(val, node_1_local_trust, 1e-9));
    /// }
    ///
    /// if let Some(val) = hp.get_raw_local(&"node1".to_string()) {
    ///     assert!(equal_floats(val, 0.06, 1e-9));
    /// }
    ///
    /// if let Some(val) = hp.get_normalized_local(&"node2".to_string()) {
    ///     assert!(equal_floats(val, node_2_local_trust, 1e-9));
    /// }
    ///
    /// if let Some(val) = hp.get_raw_local(&"node2".to_string()) {
    ///     assert!(equal_floats(val, 0.01, 1e-9));
    /// }
    ///
    /// ```
    fn update_local(&mut self, key: &Self::Key, trust_delta: Self::Value) {
        if let Some(trust_score) = self.local_trust.get_mut(key) {
            *trust_score += trust_delta
        } else {
            self.local_trust.insert(key.clone(), trust_delta);
        }

        self.normalize_local()
    }

    /// gets a value from the raw local trust map
    fn get_raw_local(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.local_trust.get(key) {
            return Some(*val)
        } 

        return None 
    }

    /// gets a value from the normalized local trust map
    fn get_normalized_local(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.normalized_local_trust.get(key) {
            return Some(*val)
        }
        return None
    }

    /// Initialize the local trust score of a newly discovered peer 
    ///
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    /// 
    /// let mut hp: PreciseHonestPeer<String, f64> = PreciseHonestPeer::new();
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_global("node1".to_string(), 0.01);
    ///
    /// assert_eq!(hp.global_raw_len(), 1);
    /// assert_eq!(hp.global_normalized_len(), 1);
    ///
    /// ```
    fn init_global(&mut self, key: &Self::Key, init_value: Self::Value) {
        self.global_trust.insert(key.clone(), init_value);
        self.normalize_global()
    }

    /// Updates a global trust value for a given peer and normalizes
    /// the normalized global trust map.
    /// ```
    /// use decentrust::precise::PreciseHonestPeer;
    ///
    /// fn equal_floats(a: &f64, b: f64, epsilon: f64) -> bool {
    ///     (*a - b).abs() < epsilon
    /// }
    ///
    /// let mut hp: PreciseHonestPeer<String, f64> = PreciseHonestPeer::new();
    /// 
    /// // Insert and normalize initial trust scores
    /// hp.init_global("node1".to_string(), 0.01);
    /// hp.init_global("node2".to_string(), 0.01);
    ///
    /// hp.update_global(&"node1".to_string(), 0.02);
    ///
    /// let global_total_trust = 0.01 + 0.01 + 0.02;
    /// let node_1_global_trust = 0.03 / global_total_trust;
    /// let node_2_global_trust = 0.01 / global_total_trust;
    ///
    /// // Since the init calls are the first in this instance,
    /// // normalization will necessarily mean that they equal 100% of the 
    /// // weight of the scores.
    /// if let Some(val) = hp.get_normalized_global(&"node1".to_string()) {
    ///     assert!(equal_floats(val, node_1_global_trust, 1e-9));
    /// }
    ///
    /// if let Some(val) = hp.get_raw_global(&"node1".to_string()) {
    ///     assert!(equal_floats(val, 0.03, 1e-9));
    /// }
    ///
    ///
    /// if let Some(val) = hp.get_normalized_global(&"node2".to_string()) {
    ///     assert!(equal_floats(val, node_2_global_trust, 1e-9));
    /// }
    ///
    /// if let Some(val) = hp.get_raw_global(&"node2".to_string()) {
    ///     assert!(equal_floats(val, 0.01, 1e-9));
    /// }
    ///
    /// ```
    fn update_global(&mut self, key: &Self::Key, trust_delta: Self::Value) {
        if let Some(trust_score) = self.global_trust.get_mut(key) {
            *trust_score += trust_delta
        } else {
            self.global_trust.insert(key.clone(), trust_delta);
        }

        self.normalize_global();
    }

    /// gets the raw global trust value for a given peer
    fn get_raw_global(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.global_trust.get(key) {
            return Some(*val)
        }

        return None

    }

    /// gets the normalized global trust value for a given peer
    fn get_normalized_global(&self, key: &Self::Key) -> Option<Self::Value> {
        if let Some(val) = self.normalized_global_trust.get(key) {
            return Some(*val)
        }

        return None
    }

    /// returns the entire raw local trust map from the `PreciseHonestPeer` instance
    fn get_raw_local_map(&self) -> Self::Map {
        self.local_trust.clone()
    }

    /// returns the entire normalized local trust map from the `PreciseHonestPeer` instance
    fn get_normalized_local_map(&self) -> Self::Map {
        self.normalized_local_trust.clone()
    }

    /// returns the entire raw global trust map from the `PreciseHonestPeer` instance
    fn get_raw_global_map(&self) -> Self::Map {

        self.global_trust.clone()
    }

    /// returns the entire normalized global trust map from the `PreciseHonestPeer` instance
    fn get_normalized_global_map(&self) -> Self::Map {
        self.normalized_global_trust.clone()
    }

    /// normalizes all the local trust values after a new entry or update 
    /// to an existing entry, and saves them in the `normalized_local_trust` 
    /// map.
    fn normalize_local(&mut self) {
        let total_trust = self.local_trust.values()
            .cloned()
            .fold(V::default(), |acc, x| acc + x);

        self.local_trust.iter().for_each(|(k, v)| {
            let normalized_trust = *v / total_trust;
            self.normalized_local_trust.insert(k.clone(), normalized_trust);
        });
    }

    /// normalizes all the global trust values after a new entry of update 
    /// to an existing entry and saves them in the `normalized_global_trust`
    /// map
    fn normalize_global(&mut self) {
        let total_trust = self.global_trust.values()
            .cloned()
            .fold(V::default(), |acc, x| acc + x);

        self.global_trust.iter_mut().for_each(|(k, v)| {
            let normalized_trust = *v / total_trust;
            self.normalized_global_trust.insert(k.clone(), normalized_trust);
        });
    }

    /// returns the number of key, value pairs in the raw local trust map 
    fn local_raw_len(&self) -> usize {
        self.local_trust.len()
    }

    /// returns the number of key, value pairs in the normalized local trust map 
    fn local_normalized_len(&self) -> usize {
        self.normalized_local_trust.len()
    }

    /// returns the number of key, value pairs in the raw global trust map 
    fn global_raw_len(&self) -> usize {
        self.global_trust.len()
    }

    /// returns the number of key, value pairs in the normalized global trust map 
    fn global_normalized_len(&self) -> usize {
        self.normalized_global_trust.len()
    }
}
