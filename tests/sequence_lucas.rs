use numberlab::sequence::lucas::{lucas_sequence, nth_lucas, nth_lucas_memoized};

#[test]
fn should_generate_nth_lucas() {
    assert_eq!(nth_lucas(0), 2);
    assert_eq!(nth_lucas(1), 1);
    assert_eq!(nth_lucas(7), 29);
    assert_eq!(nth_lucas(8), 47);
    assert_eq!(nth_lucas(9), 76);
    assert_eq!(nth_lucas(10), 123);
}

#[test]
fn should_generate_nth_lucas_with_memoization() {
    assert_eq!(nth_lucas_memoized(0), 2);
    assert_eq!(nth_lucas_memoized(1), 1);
    assert_eq!(nth_lucas_memoized(2), 3);
    assert_eq!(nth_lucas_memoized(3), 4);
    assert_eq!(nth_lucas_memoized(4), 7);
    assert_eq!(nth_lucas_memoized(5), 11);
    assert_eq!(nth_lucas_memoized(6), 18);
    assert_eq!(nth_lucas_memoized(7), 29);
    assert_eq!(nth_lucas_memoized(8), 47);
    assert_eq!(nth_lucas_memoized(9), 76);
    assert_eq!(nth_lucas_memoized(10), 123);
}

#[test]
fn should_generate_sequence_of_10_items() {
    assert_eq!(lucas_sequence(0), vec![]);
    assert_eq!(lucas_sequence(1), vec![2]);
    assert_eq!(lucas_sequence(2), vec![2, 1]);
    assert_eq!(
        lucas_sequence(10),
        (0..10).map(|n| nth_lucas(n)).collect::<Vec<_>>()
    );
}
