use numberlab::sequence::tribonacci::{
    nth_tribonacci, nth_tribonacci_memoized, tribonacci_sequence,
};

#[test]
fn should_generate_nth_tribonacci() {
    assert_eq!(nth_tribonacci(0), 0);
    assert_eq!(nth_tribonacci(1), 1);
    assert_eq!(nth_tribonacci(2), 1);
    assert_eq!(nth_tribonacci(3), 2);
    assert_eq!(nth_tribonacci(4), 4);
    assert_eq!(nth_tribonacci(5), 7);
    assert_eq!(nth_tribonacci(6), 13);
    assert_eq!(nth_tribonacci(7), 24);
    assert_eq!(nth_tribonacci(8), 44);
    assert_eq!(nth_tribonacci(9), 81);
    assert_eq!(nth_tribonacci(10), 149);
}
#[test]
fn should_generate_nth_tribonacci_memoized() {
    assert_eq!(nth_tribonacci_memoized(0), 0);
    assert_eq!(nth_tribonacci_memoized(1), 1);
    assert_eq!(nth_tribonacci_memoized(2), 1);
    assert_eq!(nth_tribonacci_memoized(3), 2);
    assert_eq!(nth_tribonacci_memoized(4), 4);
    assert_eq!(nth_tribonacci_memoized(5), 7);
    assert_eq!(nth_tribonacci_memoized(6), 13);
    assert_eq!(nth_tribonacci_memoized(7), 24);
    assert_eq!(nth_tribonacci_memoized(8), 44);
    assert_eq!(nth_tribonacci_memoized(9), 81);
    assert_eq!(nth_tribonacci_memoized(10), 149);
}

#[test]
fn should_generate_series() {
    assert_eq!(tribonacci_sequence(0), vec![]);
    assert_eq!(tribonacci_sequence(1), vec![0]);
    assert_eq!(tribonacci_sequence(2), vec![0, 1]);
    assert_eq!(tribonacci_sequence(3), vec![0, 1, 1]);
    assert_eq!(
        tribonacci_sequence(10),
        vec![0, 1, 1, 2, 4, 7, 13, 24, 44, 81]
    );
}
