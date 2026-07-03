use alloc::boxed::Box;
use std::collections::VecDeque;

/// Một Task Scheduler (Executor) đơn giản cho mô hình cooperative multitasking ``.
/// Executor này không chạy đa luồng mà chỉ lưu các closure vào queue và gọi tuần tự.
pub struct MiniExecutor {
    tasks: VecDeque<Box<dyn FnMut()>>,
}

impl MiniExecutor {
    /// Documentation for fn.
    pub const fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    /// Đưa một tác vụ vào hàng đợi
    pub fn spawn<F>(&mut self, f: F)
    where
        F: FnMut() + 'static,
    {
        self.tasks.push_back(Box::new(f));
    }

    /// Chạy 1 tác vụ. Trả về true nếu có tác vụ được chạy, false nếu queue rỗng.
    pub fn run_one(&mut self) -> bool {
        if let Some(mut task) = self.tasks.pop_front() {
            task();
            true
        } else {
            false
        }
    }

    /// Chạy tất cả các tác vụ hiện có trong queue
    pub fn run_all(&mut self) {
        while self.run_one() {}
    }
}

impl Default for MiniExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::rc::Rc;
    use core::cell::RefCell;

    #[test]
    fn test_executor_spawn_and_run() {
        let mut executor = MiniExecutor::new();
        let counter = Rc::new(RefCell::new(0));

        let c1 = counter.clone();
        executor.spawn(move || {
            *c1.borrow_mut() += 1;
        });

        let c2 = counter.clone();
        executor.spawn(move || {
            *c2.borrow_mut() += 2;
        });

        executor.run_all();

        assert_eq!(*counter.borrow(), 3);
    }
}
