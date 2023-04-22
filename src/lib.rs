pub mod precise;
pub mod probabilistic;
pub mod cms;
pub mod cms_iter;
pub mod honest_peer;

#[cfg(test)]
mod tests {
    use crate::cms::CountMinSketch;
    use crate::{
        probabilistic::LightHonestPeer,
        precise::PreciseHonestPeer,
        honest_peer::{HonestPeer, Update},
    };
    use ordered_float::OrderedFloat;
    use num_traits::Bounded;

    #[test]
    fn should_create_precise_honest_peer_instance() {
        let hp: PreciseHonestPeer<String, OrderedFloat<f64>> = PreciseHonestPeer::new();
        assert_eq!(hp.local_raw_len(), 0);
        assert_eq!(hp.global_raw_len(), 0);
    }
    
    #[test]
    fn should_create_light_honest_peer_instance() {
        let error_bound = 50.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();
        let hp: LightHonestPeer<String, OrderedFloat<f64>> = LightHonestPeer::new_from_bounds(
            error_bound, 
            probability, 
            max_entries, 
            OrderedFloat::from(min), 
            OrderedFloat::from(max)
        );

        assert_eq!(hp.get_width(), 164);
        assert_eq!(hp.get_depth(), 10);
    }

    #[test]
    fn should_create_count_min_sketch_instance() {
        let error_bound = 50.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let cms = CountMinSketch::<OrderedFloat<f64>>::new_from_bounds(
            error_bound, 
            probability, 
            max_entries, 
            OrderedFloat::from(min), 
            OrderedFloat::from(max)
        );

        assert_eq!(cms.get_width(), 164);
        assert_eq!(cms.get_depth(), 10);
    }

    #[test]
    fn should_increment_count_min_sketch_estimate() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut cms = CountMinSketch::<OrderedFloat<f64>>::new_from_bounds(
            error_bound, 
            probability, 
            max_entries, 
            OrderedFloat::from(min), 
            OrderedFloat::from(max)
        );

        cms.increment(&"node_1", 50.0.into());
        let estimate = cms.estimate(&"node_1");

        assert!(estimate >= 50.0.into() && estimate <= 60.0.into());
    }

    #[test]
    fn should_decrement_count_min_sketch_estimate() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut cms = CountMinSketch::<OrderedFloat<f64>>::new_from_bounds(
            error_bound, 
            probability, 
            max_entries, 
            OrderedFloat::from(min), 
            OrderedFloat::from(max)
        );

        cms.increment(&"node_1", 50.0.into());
        let estimate = cms.estimate(&"node_1");

        assert!(estimate >= 50.0.into() && estimate <= 60.0.into());

        cms.decrement(&"node_1", 10.0.into());

        let estimate = cms.estimate(&"node_1");

        assert!(estimate >= 40.0.into() && estimate <= 40.0.into());
    }

    #[test]
    fn should_set_value_to_zero_if_decrement_exceeds_value() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut cms = CountMinSketch::<OrderedFloat<f64>>::new_from_bounds(
            error_bound, 
            probability, 
            max_entries, 
            OrderedFloat::from(min), 
            OrderedFloat::from(max)
        );

        cms.increment(&"node_1", 50.0.into());
        let estimate = cms.estimate(&"node_1");

        assert!(estimate >= 50.0.into() && estimate <= 60.0.into());

        cms.decrement(&"node_1", 70.0.into());

        let estimate = cms.estimate(&"node_1");

        assert!(estimate >= 0.0.into() && estimate <= 10.0.into());
    }

    #[test]
    fn should_increment_node_reputation_in_precise_instance() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.update_local(&"node_1", 5.0.into(), Update::Increment);
        let trust = hp.get_raw_local(&"node_1");
        
        assert_eq!(trust, Some(OrderedFloat::from(5.0)));
        
    }

    #[test]
    fn should_decrement_node_reputation_in_precise_instance() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.update_local(&"node_1", 10.0.into(), Update::Increment);
        hp.update_local(&"node_1", 5.0.into(), Update::Decrement);
        let trust = hp.get_raw_local(&"node_1");
        
        assert_eq!(trust, Some(OrderedFloat::from(5.0)));
        
    }

    #[test]
    fn should_increment_node_reputation_in_light_instance() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.update_local(&"node_1", 50.0.into(), Update::Increment);

        let trust = hp.get_raw_local(&"node_1");

        assert_eq!(trust, Some(OrderedFloat::from(50.0)));
    }

    #[test]
    fn should_decrement_node_reputation_in_light_instance() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.update_local(&"node_1", 50.0.into(), Update::Increment);

        let trust = hp.get_raw_local(&"node_1");

        assert_eq!(trust, Some(OrderedFloat::from(50.0)));

        hp.update_local(&"node_1", 25.0.into(), Update::Decrement);

        let trust = hp.get_raw_local(&"node_1");

        assert_eq!(trust, Some(OrderedFloat::from(25.0)));
    }

    #[test]
    fn global_increment_should_be_weighted_by_sender_local_reputation_precise() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        let trust = hp.get_normalized_local(&"node_1").unwrap();
        hp.update_global(&"node_1", &"node_2", 10.0.into(), Update::Increment);

        let expected_node_2_trust = trust * OrderedFloat::from(10.0);

        let actual_node_2_trust = hp.get_raw_global(&"node_2").unwrap();

        assert_eq!(expected_node_2_trust, actual_node_2_trust);
        
    }

    #[test]
    fn decrement_should_be_weighted_by_sender_local_reputation_precise() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        let trust = hp.get_normalized_local(&"node_1").unwrap();
        hp.update_global(&"node_1", &"node_2", 10.0.into(), Update::Increment);

        let mut expected_node_2_trust = trust * OrderedFloat::from(10.0);

        let mut actual_node_2_trust = hp.get_raw_global(&"node_2").unwrap();

        assert_eq!(expected_node_2_trust, actual_node_2_trust);

        hp.update_global(&"node_1", &"node_2", 5.0.into(), Update::Decrement);
        
        let trust = hp.get_normalized_local(&"node_1").unwrap();

        expected_node_2_trust -= trust * OrderedFloat::from(5.0);

        actual_node_2_trust = hp.get_raw_global(&"node_2").unwrap();

        assert_eq!(expected_node_2_trust, actual_node_2_trust);
        
    }

    #[test]
    fn increment_should_be_weighted_by_sender_local_reputation_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        let trust = hp.get_normalized_local(&"node_1").unwrap();
        hp.update_global(&"node_1", &"node_2", 10.0.into(), Update::Increment);

        let expected_node_2_trust = trust * OrderedFloat::from(10.0);

        let actual_node_2_trust = hp.get_raw_global(&"node_2").unwrap();

        assert_eq!(expected_node_2_trust, actual_node_2_trust);
    }

    #[test]
    fn decrement_should_be_weighted_by_sender_local_reputation_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        let trust = hp.get_normalized_local(&"node_1").unwrap();
        hp.update_global(&"node_1", &"node_2", 10.0.into(), Update::Increment);

        let mut expected_node_2_trust = trust * OrderedFloat::from(10.0);

        let mut actual_node_2_trust = hp.get_raw_global(&"node_2").unwrap();

        assert_eq!(expected_node_2_trust, actual_node_2_trust);

        hp.update_global(&"node_1", &"node_2", 5.0.into(), Update::Decrement);
        
        let trust = hp.get_normalized_local(&"node_1").unwrap();

        expected_node_2_trust -= trust * OrderedFloat::from(5.0);

        actual_node_2_trust = hp.get_raw_global(&"node_2").unwrap();

        assert_eq!(expected_node_2_trust, actual_node_2_trust);
        
    }

    #[test]
    fn increments_should_be_normalized_in_normalized_local_map_precise() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.update_local(&"node_1", 10.0.into(), Update::Increment);

        let expected_node_1_norm = OrderedFloat::from(15.0) / OrderedFloat::from(20.0);
        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }

    #[test]
    fn decrements_should_be_normalized_in_normalized_local_map_precise() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.update_local(&"node_1", 2.5.into(), Update::Decrement);


        let expected_node_1_norm = {
            OrderedFloat::from(2.5) / OrderedFloat::from(7.5)
        };

        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }
    
    #[test]
    fn increments_should_be_normalized_in_normalized_global_map_precise() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.init_global(&"node_2", 2.5.into());
        hp.update_global(
            &"node_2", 
            &"node_1", 
            10.0.into(), 
            Update::Increment
        );


        let expected_node_1_norm = {
            OrderedFloat::from(5.0) / OrderedFloat::from(7.5)
        };

        let actual_node_1_norm = hp.get_normalized_global(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }

    #[test]
    fn decrements_should_be_normalized_in_normalized_global_map_precise() {
        let mut hp: PreciseHonestPeer<&str, OrderedFloat<f64>> = PreciseHonestPeer::new();

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.init_global(&"node_2", 2.5.into());
        hp.update_global(&"node_2", &"node_1", 10.0.into(), Update::Increment);
        hp.update_global(&"node_2", &"node_1", 5.0.into(), Update::Decrement);


        let expected_node_1_norm = {
            OrderedFloat::from(2.5) / OrderedFloat::from(5.0)
        };

        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }
    
    #[test]
    fn increments_should_be_normalized_in_normalized_local_map_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.update_local(&"node_1", 10.0.into(), Update::Increment);

        let expected_node_1_norm = OrderedFloat::from(15.0) / OrderedFloat::from(20.0);
        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }

    #[test]
    fn decrements_should_be_normalized_in_normalized_local_map_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.update_local(&"node_1", 2.5.into(), Update::Decrement);


        let expected_node_1_norm = {
            OrderedFloat::from(2.5) / OrderedFloat::from(7.5)
        };

        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }

    #[test]
    fn increments_should_be_normalized_in_normalized_global_map_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.init_global(&"node_2", 2.5.into());
        hp.update_global(&"node_2", &"node_1", 10.0.into(), Update::Increment);
        hp.update_global(&"node_2", &"node_1", 5.0.into(), Update::Decrement);


        let expected_node_1_norm = {
            OrderedFloat::from(2.5) / OrderedFloat::from(5.0)
        };

        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }


    #[test]
    fn decrements_should_be_normalized_in_normalized_global_map_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<&str, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        hp.init_local(&"node_1", 5.0.into());
        hp.init_local(&"node_2", 5.0.into());
        hp.init_global(&"node_2", 2.5.into());
        hp.update_global(&"node_2", &"node_1", 10.0.into(), Update::Increment);
        hp.update_global(&"node_2", &"node_1", 5.0.into(), Update::Decrement);


        let expected_node_1_norm = {
            OrderedFloat::from(2.5) / OrderedFloat::from(5.0)
        };

        let actual_node_1_norm = hp.get_normalized_local(&"node_1").unwrap();

        assert_eq!(expected_node_1_norm, actual_node_1_norm);
    }

    #[test]
    fn reputation_estimates_should_not_exceed_bounds_light() {
        let error_bound = 10.0;
        let probability = 0.0001;
        let max_entries = 3000.0;
        let min = 0.0;
        let max = f64::max_value();

        let mut hp: LightHonestPeer<usize, OrderedFloat<f64>> = {
            LightHonestPeer::new_from_bounds(
                error_bound, 
                probability, 
                max_entries, 
                OrderedFloat::from(min), 
                OrderedFloat::from(max),
            )
        };

        let nodes: Vec<usize> = (0..3000).map(|i| {
            i
        }).collect(); 

        nodes.iter().for_each(|n| {
            hp.update_local(n, 50.0.into(), Update::Increment);
        });

        nodes.iter().for_each(|n| {
            hp.update_local(n, 50.0.into(), Update::Increment);
        });

        let mut estimates: Vec<OrderedFloat<f64>> = nodes.iter().map(|n| {
            hp.get_raw_local(n).unwrap()
        }).collect();

        estimates.retain(|v| {
            v >= &OrderedFloat::from(100.0) && v <= &OrderedFloat::from(110.0)
        });
    }
}
