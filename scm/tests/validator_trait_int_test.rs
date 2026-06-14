//! Integration tests for the [`Validator`] trait.

use swe_edge_runtime_scheduler::Validator;

// ── Validator::validate ───────────────────────────────────────────────────────

/// @covers: Validator::validate
#[test]
fn test_validate_accepts_valid_impl_happy() {
    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(AlwaysValid.validate().is_ok());
}

/// @covers: Validator::validate
#[test]
fn test_validate_returns_descriptive_message_on_failure_error() {
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self) -> Result<(), String> {
            Err("value is invalid".into())
        }
    }
    match AlwaysInvalid.validate() {
        Ok(_) => panic!("expected AlwaysInvalid to return Err"),
        Err(msg) => assert!(!msg.is_empty()),
    }
}

/// @covers: Validator::validate
#[test]
fn test_validate_is_deterministic_on_repeated_calls_edge() {
    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    let v = AlwaysValid;
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}
