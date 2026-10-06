use integration_testing::public;

use integration_testing::secret;

#[test]
fn test_secret() {
    assert_eq!(secret(), 50);
}

#[test]
fn test_public() {
    assert_eq!(public(), 100);
}
