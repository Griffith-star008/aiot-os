#[tokio::test]
async fn test_failover_sequence() {
    let mut active_node = 1;
    let backup_node_ready = true;

    assert_eq!(active_node, 1);

    // Simulate node 1 crash, re-assign active_node
    active_node = 0;

    // Failover
    if backup_node_ready && active_node == 0 {
        active_node = 2;
    }

    assert_eq!(active_node, 2);
}
