use numberlab::sequence::fibonacci::{fibonacci_sequence, nth_fibonacci, nth_fibonacci_memoized};

#[test]
fn should_generate_nth_fibonacci() {
    assert_eq!(nth_fibonacci(0), 0);
    assert_eq!(nth_fibonacci(1), 1);
    assert_eq!(nth_fibonacci(2), 1);
    assert_eq!(nth_fibonacci(3), 2);
    assert_eq!(nth_fibonacci(4), 3);
    assert_eq!(nth_fibonacci(5), 5);
    assert_eq!(nth_fibonacci(10), 55);
}

#[test]
fn should_generate_nth_fibonacci_memoized() {
    assert_eq!(nth_fibonacci_memoized(0), 0);
    assert_eq!(nth_fibonacci_memoized(1), 1);
    assert_eq!(nth_fibonacci_memoized(2), 1);
    assert_eq!(nth_fibonacci_memoized(3), 2);
    assert_eq!(nth_fibonacci_memoized(4), 3);
    assert_eq!(nth_fibonacci_memoized(5), 5);
    assert_eq!(nth_fibonacci_memoized(10), 55);
}

#[test]
fn should_generate_sequence() {
    assert_eq!(fibonacci_sequence(0), vec![]);
    assert_eq!(fibonacci_sequence(1), vec![0]);
    assert_eq!(fibonacci_sequence(2), vec![0, 1]);
    assert_eq!(fibonacci_sequence(3), vec![0, 1, 1]);
    assert_eq!(
        fibonacci_sequence(10),
        (0..10).map(|n| nth_fibonacci(n)).collect::<Vec<_>>()
    );
}
