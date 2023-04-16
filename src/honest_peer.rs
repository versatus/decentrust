use std::ops::{
    AddAssign,
    DivAssign,
    SubAssign,
    Add,
    Sub,
    Div,
    Mul
};

use std::hash::Hash;

use num_traits::Bounded;

pub trait HonestPeer {
    type Map: IntoIterator;
    type Key: Eq + Hash + Clone;
    type Value: AddAssign 
        + DivAssign 
        + SubAssign 
        + Add<Output = Self::Value> 
        + Mul<Output = Self::Value> 
        + Div<Output = Self::Value> 
        + Sub<Output = Self::Value> 
        + Copy 
        + Default 
        + Bounded 
        + Hash 
        + Ord;

    fn init_local(&mut self, key: &Self::Key, init_value: Self::Value);
    fn update_local(&mut self, key: &Self::Key, trust_delta: Self::Value);
    fn get_raw_local(&self, key: &Self::Key) -> Option<Self::Value>;
    fn get_normalized_local(&self, key: &Self::Key) -> Option<Self::Value>;
    fn init_global(&mut self, key: &Self::Key, init_value: Self::Value);
    fn update_global(&mut self, key: &Self::Key, trust_delta: Self::Value);
    fn get_raw_global(&self, key: &Self::Key) -> Option<Self::Value>;
    fn get_normalized_global(&self, key: &Self::Key) -> Option<Self::Value>;
    fn get_raw_local_map(&self) -> Self::Map;
    fn get_normalized_local_map(&self) -> Self::Map;
    fn get_raw_global_map(&self) -> Self::Map;
    fn get_normalized_global_map(&self) -> Self::Map;
    fn normalize_local(&mut self);
    fn normalize_global(&mut self);
    fn local_raw_len(&self) -> usize;
    fn local_normalized_len(&self) -> usize;
    fn global_raw_len(&self) -> usize;
    fn global_normalized_len(&self) -> usize;
}