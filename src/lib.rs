pub mod precise;
pub mod probabilistic;
pub mod cms;
pub mod cms_iter;
pub mod honest_peer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilistic::LightHonestPeer;
    use crate::honest_peer::HonestPeer;
    use buckets::bucketizers::{fw::FixedWidthBucketizer, range::RangeBucketizer};
    use buckets::bucketize::BucketizeSingle;
    use ordered_float::OrderedFloat;
    use num_traits::Bounded;
    

    #[test]
    fn should_create_precise_honest_peer_instance() {}
    
    #[test]
    fn should_create_light_honest_peer_instance() {}

    #[test]
    fn should_create_count_min_sketch_instance() {}

    #[test]
    fn should_increment_count_min_sketch_estimate() {}

    #[test]
    fn should_decrement_count_min_sketch_estimate() {}

    #[test]
    fn should_increment_node_reputation_in_precise_instance() {}

    #[test]
    fn should_decrement_node_reputation_in_precise_instance() {}

    #[test]
    fn should_increment_node_reputation_in_light_instance() {}

    #[test]
    fn should_decrement_node_reputation_in_light_instance() {}

    #[test]
    fn increment_should_be_weighted_by_sender_local_reputation_precise() {}

    #[test]
    fn decrement_should_be_weighted_by_sender_local_reputation_precise() {}

    #[test]
    fn increment_should_be_weighted_by_sender_local_reputation_light() {}

    #[test]
    fn decrement_should_be_weighted_by_sender_local_reputation_light() {}

    #[test]
    fn increments_should_be_normalized_in_normalized_local_map_precise() {}

    #[test]
    fn decrements_should_be_normalized_in_normalized_global_map_precise() {}
    
    #[test]
    fn increments_should_be_normalized_in_normalized_local_map_light() {}

    #[test]
    fn decrements_should_be_normalized_in_normalized_global_map_light() {}

    #[test]
    fn reputation_estimates_should_not_exceed_bounds_light() {}

    #[test]
    fn reputation_scores_should_be_precise_in_precise() {}

    #[test]
    fn reputation_estimates_should_maintain_insertion_order_light() {}

    #[test]
    fn should_get_nodes_reputation_score_precise() {}

    #[test]
    fn should_bucketize_estimates() {

        let mut hp: LightHonestPeer<String, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                1f64,
                0.0001f64,
                3000f64,
                OrderedFloat::<f64>::min_value(),
                OrderedFloat::<f64>::max_value()
            )
        };

        let node_ids = vec!["node_1".to_string(), "node_2".to_string(), "abcde".to_string()];
        let ranges: Vec<(OrderedFloat<f64>, OrderedFloat<f64>)> = vec![
            (OrderedFloat::from(0.0), OrderedFloat::from(5.0)),
            (OrderedFloat::from(5.0), OrderedFloat::from(15.0)), 
            (OrderedFloat::from(15.0), OrderedFloat::from(30.0)),
            (OrderedFloat::from(30.0), OrderedFloat::<f64>::max_value())
        ];

        let bucketizer = RangeBucketizer::new(ranges); 

        hp.update_local(&"node_1".to_string(), OrderedFloat::from(7.0));
        hp.update_local(&"node_2".to_string(), OrderedFloat::from(3.0));

        let mut map = hp.bucketize_local(node_ids.iter().cloned(), bucketizer);

        assert_eq!(Some(("node_1".to_string(), 1)), map.next());
        assert_eq!(Some(("node_2".to_string(), 0)), map.next());
        assert_eq!(Some(("abcde".to_string(), 0)), map.next());

    }

    #[test]
    fn should_bucketize_normalized_estimates() {
        let mut hp: LightHonestPeer<String, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                1f64,
                0.0001f64,
                3000f64,
                OrderedFloat::<f64>::min_value(),
                OrderedFloat::<f64>::max_value()
            )
        };
    
        let node_ids = vec!["node_1".to_string(), "node_2".to_string(), "abcde".to_string()];
    
        let bucketizer: FixedWidthBucketizer<OrderedFloat<f64>> = {
            FixedWidthBucketizer::<OrderedFloat<f64>>::new(
                OrderedFloat::from(0.05), OrderedFloat::from(0.0)
            ) 
        };
    
        hp.update_local(&"node_1".to_string(), OrderedFloat::from(7.0));
        hp.update_local(&"node_2".to_string(), OrderedFloat::from(3.0));

        let mut map = hp.bucketize_normalized_local(node_ids.iter().cloned(), bucketizer);
    
        assert_eq!(Some(("node_1".to_string(), 13)), map.next());
        assert_eq!(Some(("node_2".to_string(), 5)), map.next());
        assert_eq!(Some(("abcde".to_string(), 0)), map.next());
    }
}
