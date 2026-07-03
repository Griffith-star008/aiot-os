use runtime::{RuntimeBuilder, LifecycleState};

fn main() {
    println!("Welcome to AIoT Framework: Hello World Example");
    
    // Khởi tạo runtime bằng Builder
    let mut runtime = RuntimeBuilder::new()
        .with_initial_state(LifecycleState::Created)
        .build();

    println!("Runtime initialized. Current state: {:?}", runtime.current_state);

    // Boot quá trình khởi động
    runtime.transition_to(LifecycleState::Running).expect("Failed to start runtime");
    println!("Runtime started. Current state: {:?}", runtime.current_state);
    
    println!("Hello World completed successfully.");
}
