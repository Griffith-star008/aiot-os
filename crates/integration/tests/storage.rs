use queue::{InMemoryPersistentQueue, PersistentTaskQueue, QueuedTask};

#[test]
fn test_persistent_queue_storage() {
    let mut queue = InMemoryPersistentQueue::new(10);
    let task = QueuedTask {
        task_id: 1,
        payload: vec![1, 2, 3],
        priority: 0,
    };

    assert!(queue.enqueue(task).is_ok());

    let dequeued = queue.dequeue().unwrap();
    assert!(dequeued.is_some());
    assert_eq!(dequeued.unwrap().task_id, 1);

    assert!(queue.ack(1).is_ok());
}
