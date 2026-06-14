//! Integration tests for the validator SAF layer.

use swe_edge_runtime_scheduler::VALIDATOR_LAYER;

/// @covers: VALIDATOR_LAYER
#[test]
fn test_validator_svc_layer_constant_identifies_facade() {
    assert_eq!(VALIDATOR_LAYER, "validator");
}
