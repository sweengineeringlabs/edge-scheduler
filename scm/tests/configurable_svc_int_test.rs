//! Integration tests for the configurable SAF layer.

use swe_edge_runtime_scheduler::CONFIGURABLE_LAYER;

/// @covers: CONFIGURABLE_LAYER
#[test]
fn test_configurable_svc_layer_constant_identifies_facade() {
    assert_eq!(CONFIGURABLE_LAYER, "configurable");
}
