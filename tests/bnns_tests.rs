use apple_accelerate::{
    bnns_graph_optimization_preference, bnns_relu_f32, bnns_sigmoid_f32, BnnsGraphCompileOptions,
};

#[test]
fn bnns_activation_helpers_smoke() {
    let relu = bnns_relu_f32(&[-3.0_f32, 0.5, 2.0]).expect("relu");
    assert!(relu
        .iter()
        .zip([0.0_f32, 0.5, 2.0])
        .all(|(actual, expected)| (*actual - expected).abs() < 1.0e-6));

    let sigmoid = bnns_sigmoid_f32(&[0.0_f32, 2.0]).expect("sigmoid");
    assert!((sigmoid[0] - 0.5).abs() < 1.0e-6);
    assert!((sigmoid[1] - 0.880_797).abs() < 1.0e-5);
}

#[test]
fn bnns_graph_compile_options_smoke() {
    let Some(mut options) = BnnsGraphCompileOptions::new() else {
        return;
    };

    options
        .set_target_single_thread(true)
        .expect("single thread");
    assert!(options.target_single_thread());

    options
        .set_generate_debug_info(true)
        .expect("debug info");
    assert!(options.generate_debug_info());

    options
        .set_optimization_preference(bnns_graph_optimization_preference::IR_SIZE)
        .expect("optimization preference");
    assert_eq!(
        options.optimization_preference(),
        bnns_graph_optimization_preference::IR_SIZE
    );
}
