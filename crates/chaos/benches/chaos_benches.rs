use chaos::ChaosMonkey;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use safety::{FaultType, SafetyMonitor};

fn bench_chaos_injection(c: &mut Criterion) {
    c.bench_function("inject_chaos_under_load", |b| {
        b.iter(|| {
            let mut monkey = ChaosMonkey::new();
            monkey.enable();
            let mut monitor = SafetyMonitor::new();

            // Giả lập bắn nhiệt độ
            let mut temp = 50;
            monkey.inject_thermal_spike(&mut temp);

            if temp > 85 {
                monitor.report_fault(FaultType::OverVoltage); // Fallback fault
            }

            // Giả lập lật bit memory
            let mut mem_flag: u32 = 0;
            monkey.inject_bitflip(&mut mem_flag);
            if mem_flag != 0 {
                monitor.report_fault(FaultType::MemoryCorruption);
            }

            black_box(monitor.get_overall_risk(temp));
            black_box(monitor.evaluate_system_state());
        })
    });
}

criterion_group!(benches, bench_chaos_injection);
criterion_main!(benches);
