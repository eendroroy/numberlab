use numberlab::sequence::sylvester::{nth_sylvester, nth_sylvester_memoized, sylvester_sequence};

#[test]
fn should_generate_nth_sylvester() {
    assert_eq!(nth_sylvester(0), 2);
    assert_eq!(nth_sylvester(1), 3);
    assert_eq!(nth_sylvester(2), 7);
    assert_eq!(nth_sylvester(3), 43);
    assert_eq!(nth_sylvester(4), 1807);
    assert_eq!(nth_sylvester(5), 3263443);
    assert_eq!(nth_sylvester(6), 10650056950807);
    assert_eq!(nth_sylvester(7), 113423713055421844361000443);
}

#[test]
fn should_generate_nth_sylvester_memoized() {
    assert_eq!(nth_sylvester_memoized(0), 2);
    assert_eq!(nth_sylvester_memoized(1), 3);
    assert_eq!(nth_sylvester_memoized(2), 7);
    assert_eq!(nth_sylvester_memoized(3), 43);
    assert_eq!(nth_sylvester_memoized(4), 1807);
    assert_eq!(nth_sylvester_memoized(5), 3263443);
    assert_eq!(nth_sylvester_memoized(6), 10650056950807);
    assert_eq!(nth_sylvester_memoized(7), 113423713055421844361000443);
}

#[test]
fn should_generate_sequence_of_0_items() {
    assert_eq!(sylvester_sequence(0), vec![]);
}

#[test]
fn should_generate_sequence_of_1_items() {
    assert_eq!(sylvester_sequence(1), vec![2]);
}

#[test]
fn should_generate_series() {
    assert_eq!(
        sylvester_sequence(8),
        vec![2, 3, 7, 43, 1807, 3263443, 10650056950807, 113423713055421844361000443]
    );
}
