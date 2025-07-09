use data::queue::Queue;

#[test]
fn test_queue_enqueue_dequeue() {
    let mut q: Queue<u8, 4> = Queue::new();
    assert_eq!(q.len(), 0);
    assert!(q.is_empty());
    assert!(!q.is_full());

    assert_eq!(q.enqueue(10), Ok(()));
    assert_eq!(q.enqueue(20), Ok(()));
    assert_eq!(q.enqueue(30), Ok(()));
    assert_eq!(q.enqueue(40), Ok(()));
    assert!(q.is_full());
    assert_eq!(q.len(), 4);

    // Queue is full, next enqueue should fail
    assert_eq!(q.enqueue(50), Err(50));

    // Dequeue in order
    assert_eq!(q.dequeue(), Some(10));
    assert_eq!(q.dequeue(), Some(20));
    assert_eq!(q.len(), 2);
    assert!(!q.is_full());
    assert!(!q.is_empty());

    assert_eq!(q.enqueue(50), Ok(()));
    assert_eq!(q.enqueue(60), Ok(()));
    assert!(q.is_full());

    assert_eq!(q.dequeue(), Some(30));
    assert_eq!(q.dequeue(), Some(40));
    assert_eq!(q.dequeue(), Some(50));
    assert_eq!(q.dequeue(), Some(60));
    assert_eq!(q.dequeue(), None);
    assert!(q.is_empty());
}

#[test]
fn test_queue_peek_and_peek_mut() {
    let mut q: Queue<u32, 2> = Queue::new();
    assert_eq!(q.peek(), None);
    assert_eq!(q.peek_mut(), None);

    q.enqueue(100).unwrap();
    assert_eq!(q.peek(), Some(&100));
    if let Some(x) = q.peek_mut() {
        *x = 200;
    }
    assert_eq!(q.peek(), Some(&200));
    assert_eq!(q.dequeue(), Some(200));
    assert_eq!(q.peek(), None);
}

#[test]
fn test_queue_wrap_around() {
    let mut q: Queue<u8, 3> = Queue::new();
    q.enqueue(1).unwrap();
    q.enqueue(2).unwrap();
    q.enqueue(3).unwrap();
    assert!(q.is_full());

    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert!(!q.is_full());
    assert!(!q.is_empty());

    q.enqueue(4).unwrap();
    q.enqueue(5).unwrap();
    assert!(q.is_full());

    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), Some(4));
    assert_eq!(q.dequeue(), Some(5));
    assert_eq!(q.dequeue(), None);
    assert!(q.is_empty());
}

#[test]
fn test_queue_capacity() {
    let q: Queue<u8, 7> = Queue::new();
    assert_eq!(q.capacity(), 7);
}
