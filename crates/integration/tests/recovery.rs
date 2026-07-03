use resilience::{CircuitBreaker, CircuitState};

#[tokio::test]
async fn test_circuit_breaker_recovery() {
    let mut cb = CircuitBreaker::new(3, 2, 100);
    cb.record_failure(10);
    cb.record_failure(20);
    cb.record_failure(30);

    assert_eq!(cb.state, CircuitState::Open);

    // Attempt after timeout
    let res = cb.allow_request(150);
    assert!(res.is_ok());
    assert_eq!(cb.state, CircuitState::HalfOpen);

    cb.record_success();
    cb.record_success();
    assert_eq!(cb.state, CircuitState::Closed);
}
