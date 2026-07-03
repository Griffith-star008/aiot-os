use runtime::{LifecycleState, RuntimeLifecycle};

#[test]
fn test_startup_sequence() {
    let mut lifecycle = RuntimeLifecycle::new();
    assert_eq!(lifecycle.current_state, LifecycleState::Created);

    let res = lifecycle.transition_to(LifecycleState::Configured);
    assert!(res.is_ok());

    let res = lifecycle.transition_to(LifecycleState::Initialized);
    assert!(res.is_ok());

    let res = lifecycle.transition_to(LifecycleState::Running);
    assert!(res.is_ok());
}
