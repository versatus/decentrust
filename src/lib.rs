pub mod precise;
pub mod probabilistic;
pub mod cms;
pub mod cms_iter;
pub mod honest_peer;

#[cfg(test)]
mod tests {
    use super::*;

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

}
