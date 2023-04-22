use std::hash::Hash;
use std::ops::{AddAssign, DivAssign, SubAssign, Add, Mul, Div, Sub};
use buckets::bucketize::BucketizeSingle;
use num_traits::Bounded;
use std::marker::PhantomData;
use crate::cms::CountMinSketch;
use crate::honest_peer::{HonestPeer, Update};
use std::fmt::Debug;

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
/// use std::fmt::Debug;
///
/// pub struct LightHonestPeer<K, V> 
/// where 
///     K: Eq + Hash + Clone + Debug + ToString,
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
///     + Debug
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
    K: Eq + Hash + Clone + Debug + ToString,
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
    + Debug
{
    local_trust: CountMinSketch<V>,
    global_trust: CountMinSketch<V>,
    normalized_local_trust: CountMinSketch<V>,
    normalized_global_trust: CountMinSketch<V>,
    pub id_type: Option<PhantomData<K>>
}


impl<K, V> LightHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone + Debug + ToString,
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
    + Debug
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

    /// Iterates over provided ids, and returns an iterator over 
    /// (id, usize), i.e. the identifier for each item 
    /// and the bucketized estimate for that item in the raw local 
    /// `CountMinSketch`
    ///
    /// # Example 
    ///
    /// ```
    ///
    /// use decentrust::probabilistic::LightHonestPeer;
    /// use decentrust::honest_peer::{HonestPeer, Update};
    /// use buckets::bucketizers::range::RangeBucketizer;
    /// use buckets::bucketize::BucketizeSingle;
    /// use ordered_float::OrderedFloat;
    /// use num_traits::Bounded;
    ///
    /// let mut hp: LightHonestPeer<String, OrderedFloat<f64>> = {
    ///    LightHonestPeer::new_from_bounds(
    ///        1f64,
    ///        0.0001f64,
    ///        3000f64,
    ///        OrderedFloat::<f64>::min_value(),
    ///        OrderedFloat::<f64>::max_value()
    ///    )
    /// };
    ///
    /// let node_ids = vec!["node_1".to_string(), "node_2".to_string(), "abcde".to_string()];
    /// let ranges: Vec<(OrderedFloat<f64>, OrderedFloat<f64>)> = vec![
    ///     (OrderedFloat::from(0.0), OrderedFloat::from(5.0)),
    ///     (OrderedFloat::from(5.0), OrderedFloat::from(15.0)), 
    ///     (OrderedFloat::from(15.0), OrderedFloat::from(30.0)),
    ///     (OrderedFloat::from(30.0), OrderedFloat::<f64>::max_value())
    /// ];
    ///
    /// let bucketizer = RangeBucketizer::new(ranges); 
    ///
    /// hp.update_local(&"node_1".to_string(), OrderedFloat::from(7.0), Update::Increment);
    /// hp.update_local(&"node_2".to_string(), OrderedFloat::from(3.0), Update::Increment);
    ///
    /// let mut map = hp.bucketize_local(node_ids.iter().cloned(), bucketizer);
    ///
    /// assert_eq!(Some(("node_1".to_string(), 1)), map.next());
    /// assert_eq!(Some(("node_2".to_string(), 0)), map.next());
    /// assert_eq!(Some(("abcde".to_string(), 0)), map.next());
    /// ```
    pub fn bucketize_local<'a, B>(
        &'a self, 
        node_ids: impl Iterator<Item = K> + 'a, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
       node_ids.map(move |k| {
            let estimate = self.local_trust.estimate(&k);
            let bucketed = bucketizer.bucketize(&estimate);
            (k, bucketed)

        })
    }

    /// Iterates over provided ids, and returns an iterator over 
    /// (id, usize), i.e. the identifier for each item 
    /// and the bucketized estimate for that item in the normalized local 
    /// `CountMinSketch`
    ///
    /// # Example 
    ///
    /// ```
    /// use decentrust::probabilistic::LightHonestPeer;
    /// use decentrust::honest_peer::{HonestPeer, Update};
    /// use buckets::bucketizers::fw::FixedWidthBucketizer;
    /// use buckets::bucketize::BucketizeSingle;
    /// use buckets::into_usize::IntoUsize;
    /// use ordered_float::OrderedFloat;
    /// use num_traits::Bounded;
    ///
    /// let mut hp: LightHonestPeer<String, OrderedFloat<f64>> = {
    ///    LightHonestPeer::new_from_bounds(
    ///        1f64,
    ///        0.0001f64,
    ///        3000f64,
    ///        OrderedFloat::<f64>::min_value(),
    ///        OrderedFloat::<f64>::max_value()
    ///    )
    /// };
    ///
    /// let node_ids = vec!["node_1".to_string(), "node_2".to_string(), "abcde".to_string()];
    ///
    /// let bucketizer: FixedWidthBucketizer<OrderedFloat<f64>> = {
    ///     FixedWidthBucketizer::<OrderedFloat<f64>>::new(
    ///         OrderedFloat::from(0.05), OrderedFloat::from(0.0)
    ///     ) 
    /// };
    ///
    /// hp.update_local(&"node_1".to_string(), OrderedFloat::from(7.0), Update::Increment);
    /// hp.update_local(&"node_2".to_string(), OrderedFloat::from(3.0), Update::Increment);
    ///
    /// let mut map = hp.bucketize_normalized_local(node_ids.iter().cloned(), bucketizer);
    ///
    /// assert_eq!(Some(("node_1".to_string(), 13)), map.next());
    /// assert_eq!(Some(("node_2".to_string(), 5)), map.next());
    /// assert_eq!(Some(("abcde".to_string(), 0)), map.next());
    ///
    /// ```
    pub fn bucketize_normalized_local<'a, B>(
        &'a self, 
        node_ids: impl Iterator<Item = K> + 'a, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        node_ids.map(move |k| {
            let estimate = self.normalized_local_trust.estimate(&k);
            let bucketed = bucketizer.bucketize(&estimate);
            (k, bucketed)

        })
    }

    /// Iterates over provided ids, and returns an iterator over 
    /// (id, usize), i.e. the identifier for each item 
    /// and the bucketized estimate for that item in the raw global 
    /// `CountMinSketch`
    ///
    /// # Example 
    ///
    /// ```
    /// use decentrust::probabilistic::LightHonestPeer;
    /// use decentrust::honest_peer::{HonestPeer, Update};
    /// use buckets::bucketizers::range::RangeBucketizer;
    /// use buckets::bucketize::BucketizeSingle;
    /// use ordered_float::OrderedFloat;
    /// use num_traits::Bounded;
    ///
    /// let mut hp: LightHonestPeer<String, OrderedFloat<f64>> = {
    ///    LightHonestPeer::new_from_bounds(
    ///        1f64,
    ///        0.0001f64,
    ///        3000f64,
    ///        OrderedFloat::<f64>::min_value(),
    ///        OrderedFloat::<f64>::max_value()
    ///    )
    /// };
    ///
    /// let node_ids = vec!["node_1".to_string(), "node_2".to_string(), "abcde".to_string()];
    /// let ranges: Vec<(OrderedFloat<f64>, OrderedFloat<f64>)> = vec![
    ///     (OrderedFloat::from(0.0), OrderedFloat::from(5.0)),
    ///     (OrderedFloat::from(5.0), OrderedFloat::from(15.0)), 
    ///     (OrderedFloat::from(15.0), OrderedFloat::from(30.0)),
    ///     (OrderedFloat::from(30.0), OrderedFloat::<f64>::max_value())
    /// ];
    ///
    /// let bucketizer = RangeBucketizer::new(ranges); 
    /// hp.init_local(&"node_1".to_string(), OrderedFloat::from(1.0));
    /// hp.init_local(&"node_2".to_string(), OrderedFloat::from(1.0));
    ///
    /// hp.update_global(
    ///     &"node_2".to_string(), 
    ///     &"node_1".to_string(), 
    ///     OrderedFloat::from(14.0), 
    ///     Update::Increment
    /// );
    ///
    /// hp.update_global(
    ///     &"node_1".to_string(), 
    ///     &"node_2".to_string(), 
    ///     OrderedFloat::from(6.0), 
    ///     Update::Increment
    /// );
    ///
    /// let mut map = hp.bucketize_global(node_ids.iter().cloned(), bucketizer);
    ///
    /// assert_eq!(Some(("node_1".to_string(), 1)), map.next());
    /// assert_eq!(Some(("node_2".to_string(), 0)), map.next());
    /// assert_eq!(Some(("abcde".to_string(), 0)), map.next());
    /// ```
    pub fn bucketize_global<'a, B>(
        &'a self, 
        node_ids: impl Iterator<Item = K> + 'a, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        node_ids.map(move |k| {
            let estimate = self.global_trust.estimate(&k);
            let bucketed = bucketizer.bucketize(&estimate);
            (k, bucketed)

        })
    }

    /// Iterates over provided ids, and returns an iterator over 
    /// (id, usize), i.e. the identifier for each item 
    /// and the bucketized estimate for that item in the normalized global 
    /// `CountMinSketch`
    ///
    /// # Example 
    ///
    /// ```
    ///
    /// use decentrust::probabilistic::LightHonestPeer;
    /// use decentrust::honest_peer::{HonestPeer, Update};
    /// use buckets::bucketizers::fw::FixedWidthBucketizer;
    /// use buckets::bucketize::BucketizeSingle;
    /// use buckets::into_usize::IntoUsize;
    /// use ordered_float::OrderedFloat;
    /// use num_traits::Bounded;
    ///
    /// let mut hp: LightHonestPeer<String, OrderedFloat<f64>> = {
    ///    LightHonestPeer::new_from_bounds(
    ///        1f64,
    ///        0.0001f64,
    ///        3000f64,
    ///        OrderedFloat::<f64>::min_value(),
    ///        OrderedFloat::<f64>::max_value()
    ///    )
    /// };
    ///
    /// let node_ids = vec!["node_1".to_string(), "node_2".to_string(), "abcde".to_string()];
    ///
    /// let bucketizer: FixedWidthBucketizer<OrderedFloat<f64>> = {
    ///     FixedWidthBucketizer::<OrderedFloat<f64>>::new(
    ///         OrderedFloat::from(0.05), OrderedFloat::from(0.0)
    ///     ) 
    /// };
    ///
    /// hp.init_local(&"node_1".to_string(), OrderedFloat::from(1.0));
    /// hp.init_local(&"node_2".to_string(), OrderedFloat::from(1.0));
    ///
    /// hp.update_global(
    ///     &"node_2".to_string(), 
    ///     &"node_1".to_string(), 
    ///     OrderedFloat::from(14.0), 
    ///     Update::Increment
    /// );
    ///
    /// hp.update_global(
    ///     &"node_1".to_string(), 
    ///     &"node_2".to_string(), 
    ///     OrderedFloat::from(6.0), 
    ///     Update::Increment
    /// );
    ///
    /// let mut map = hp.bucketize_normalized_global(node_ids.iter().cloned(), bucketizer);
    ///
    /// assert_eq!(Some(("node_1".to_string(), 13)), map.next());
    /// assert_eq!(Some(("node_2".to_string(), 5)), map.next());
    /// assert_eq!(Some(("abcde".to_string(), 0)), map.next());
    ///
    /// ```
    pub fn bucketize_normalized_global<'a, B>(
        &'a self, 
        node_ids: impl Iterator<Item = K> + 'a, 
        bucketizer: B
    ) -> impl Iterator<Item = (K, usize)> + '_
    where 
        B: BucketizeSingle<V> + 'a
    {
        node_ids.map(move |k| {
            let estimate = self.normalized_global_trust.estimate(&k);
            let bucketed = bucketizer.bucketize(&estimate);
            (k, bucketed)

        })
    }

    pub fn get_width(&self) -> usize {
        self.local_trust.get_width()
    }

    pub fn get_depth(&self) -> usize {
        self.local_trust.get_depth()
    }
}

impl<K, V> HonestPeer for LightHonestPeer<K, V> 
where 
    K: Eq + Hash + Clone + Debug + ToString,
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
        + Debug
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
    fn update_local(
        &mut self, 
        key: &Self::Key, 
        trust_delta: Self::Value, 
        update: Update
    ) {
        match update {
            Update::Increment => self.local_trust.increment(key, trust_delta),
            Update::Decrement => self.local_trust.decrement(key, trust_delta), 
        }
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
    fn init_global(&mut self, sender: &Self::Key, key: &Self::Key, init_value: Self::Value) {
        let sender_trust = self.normalized_local_trust.estimate(sender);
        let weighted_init = init_value * sender_trust;
        self.global_trust.increment(key, weighted_init);
        self.normalize_global();
    }

    /// updates a global trust value for a given peer
    fn update_global(
        &mut self, 
        sender: &Self::Key,
        key: &Self::Key, 
        trust_delta: Self::Value, 
        update: Update
    ) {
        let sender_trust = self.normalized_local_trust.estimate(sender);
        let weighted_delta = trust_delta * sender_trust;
        match update {
            Update::Increment => self.global_trust.increment(key, weighted_delta),
            Update::Decrement => self.global_trust.decrement(key, weighted_delta)
        }
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
