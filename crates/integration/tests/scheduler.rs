use scheduler::{AutonomousScheduler, RlSchedulerEngine, SchedulingAction};

#[test]
fn test_scheduler_rl_engine() {
    let mut scheduler = AutonomousScheduler::new();
    let state = scheduler.observe_state();

    assert_eq!(state.pending_tasks, 0);
    assert_eq!(state.cpu_utilization, 0);

    let action = scheduler.select_action(&state);
    assert_eq!(action, SchedulingAction::EnterPowerSaving);
}
