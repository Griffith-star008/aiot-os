use runtime::{LifecycleState, RuntimeLifecycle};

#[tokio::test]
async fn test_graceful_shutdown() {
    let mut lifecycle = RuntimeLifecycle::new();
    let _ = lifecycle.transition_to(LifecycleState::Running);

    let res = lifecycle.transition_to(LifecycleState::Stopping);
    assert!(res.is_ok());

    let res = lifecycle.transition_to(LifecycleState::Stopped);
    assert!(res.is_ok());
}
