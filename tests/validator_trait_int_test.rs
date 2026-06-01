//! Integration tests for the [`Validator`] trait.

use swe_edge_runtime_scheduler::Validator;

/// @covers: Validator::validate
#[test]
fn test_validator_trait_validate_ok_path() {
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
fn test_validator_trait_validate_err_path() {
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(&self) -> Result<(), String> {
            Err("invalid".into())
        }
    }
    assert!(AlwaysInvalid.validate().is_err());
}
