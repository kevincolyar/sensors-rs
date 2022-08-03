struct State {
    value: f64,
}

#[test]
fn test_creation() {
    assert!(State { value: 0.0 }.value == 0.0);
}
