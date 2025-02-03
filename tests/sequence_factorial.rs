use numberlab::sequence::factorial::{factorial_sequence, nth_factorial, nth_factorial_memoized};

#[test]
fn should_calculate_nth_factorial() {
    assert_eq!(nth_factorial(0), 1);
    assert_eq!(nth_factorial(1), 1);
    assert_eq!(nth_factorial(2), 2);
    assert_eq!(nth_factorial(3), 6);
    assert_eq!(nth_factorial(4), 24);
    assert_eq!(nth_factorial(5), 120);
    assert_eq!(nth_factorial(6), 720);
    assert_eq!(nth_factorial(7), 5040);
    assert_eq!(nth_factorial(8), 40320);
    assert_eq!(nth_factorial(9), 362880);
    assert_eq!(nth_factorial(10), 3628800);
}

#[test]
fn should_calculate_nth_factorial_memoized() {
    assert_eq!(nth_factorial_memoized(0), 1);
    assert_eq!(nth_factorial_memoized(1), 1);
    assert_eq!(nth_factorial_memoized(2), 2);
    assert_eq!(nth_factorial_memoized(3), 6);
    assert_eq!(nth_factorial_memoized(4), 24);
    assert_eq!(nth_factorial_memoized(5), 120);
    assert_eq!(nth_factorial_memoized(6), 720);
    assert_eq!(nth_factorial_memoized(7), 5040);
    assert_eq!(nth_factorial_memoized(8), 40320);
    assert_eq!(nth_factorial_memoized(9), 362880);
    assert_eq!(nth_factorial_memoized(10), 3628800);
}

#[test]
fn should_generate_factorial_sequence() {
    assert_eq!(factorial_sequence(0), vec![]);
    assert_eq!(factorial_sequence(1), vec![1]);
    assert_eq!(
        factorial_sequence(10),
        (0..10).map(|n| nth_factorial(n)).collect::<Vec<_>>()
    );
}
