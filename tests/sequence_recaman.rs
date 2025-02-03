use numberlab::sequence::recaman::{nth_recaman_memoized, recaman_sequence};

#[test]
fn should_generate_nth_recaman() {
    assert_eq!(nth_recaman_memoized(0), 0);
    assert_eq!(nth_recaman_memoized(1), 1);
    assert_eq!(nth_recaman_memoized(2), 3);
    assert_eq!(nth_recaman_memoized(3), 6);
    assert_eq!(nth_recaman_memoized(4), 2);
    assert_eq!(nth_recaman_memoized(5), 7);
    assert_eq!(nth_recaman_memoized(6), 13);
    assert_eq!(nth_recaman_memoized(7), 20);
    assert_eq!(nth_recaman_memoized(8), 12);
    assert_eq!(nth_recaman_memoized(9), 21);
    assert_eq!(nth_recaman_memoized(14), 9);
    assert_eq!(nth_recaman_memoized(10), 11);
    assert_eq!(nth_recaman_memoized(11), 22);
    assert_eq!(nth_recaman_memoized(12), 10);
    assert_eq!(nth_recaman_memoized(13), 23);
    assert_eq!(nth_recaman_memoized(15), 24);
    assert_eq!(nth_recaman_memoized(16), 8);
    assert_eq!(nth_recaman_memoized(17), 25);
    assert_eq!(nth_recaman_memoized(18), 43);
    assert_eq!(nth_recaman_memoized(19), 62);
    assert_eq!(nth_recaman_memoized(20), 42);
    assert_eq!(nth_recaman_memoized(21), 63);
    assert_eq!(nth_recaman_memoized(22), 41);
    assert_eq!(nth_recaman_memoized(23), 18);
    assert_eq!(nth_recaman_memoized(24), 42);
    assert_eq!(nth_recaman_memoized(25), 17);
    assert_eq!(nth_recaman_memoized(26), 43);
    assert_eq!(nth_recaman_memoized(27), 16);
    assert_eq!(nth_recaman_memoized(28), 44);
    assert_eq!(nth_recaman_memoized(29), 15);
}

#[test]
fn should_generate_recaman_sequence() {
    assert_eq!(recaman_sequence(10), vec![0, 1, 3, 6, 2, 7, 13, 20, 12, 21]);
}
