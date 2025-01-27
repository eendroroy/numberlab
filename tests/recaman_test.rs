use numberlab::sequence::recaman::{nth_recaman_memoized, recaman_sequence};

#[test]
fn should_generate_nth_recaman() {
    let mut sequence = vec![];
    assert_eq!(nth_recaman_memoized(&mut sequence, 0), 0);
    assert_eq!(nth_recaman_memoized(&mut sequence, 1), 1);
    assert_eq!(nth_recaman_memoized(&mut sequence, 2), 3);
    assert_eq!(nth_recaman_memoized(&mut sequence, 3), 6);
    assert_eq!(nth_recaman_memoized(&mut sequence, 4), 2);
    assert_eq!(nth_recaman_memoized(&mut sequence, 5), 7);
    assert_eq!(nth_recaman_memoized(&mut sequence, 6), 13);
    assert_eq!(nth_recaman_memoized(&mut sequence, 7), 20);
    assert_eq!(nth_recaman_memoized(&mut sequence, 8), 12);
    assert_eq!(nth_recaman_memoized(&mut sequence, 9), 21);
    assert_eq!(nth_recaman_memoized(&mut sequence, 14), 9);
    assert_eq!(nth_recaman_memoized(&mut sequence, 10), 11);
    assert_eq!(nth_recaman_memoized(&mut sequence, 11), 22);
    assert_eq!(nth_recaman_memoized(&mut sequence, 12), 10);
    assert_eq!(nth_recaman_memoized(&mut sequence, 13), 23);
    assert_eq!(nth_recaman_memoized(&mut sequence, 15), 24);
    assert_eq!(nth_recaman_memoized(&mut sequence, 16), 8);
    assert_eq!(nth_recaman_memoized(&mut sequence, 17), 25);
    assert_eq!(nth_recaman_memoized(&mut sequence, 18), 43);
    assert_eq!(nth_recaman_memoized(&mut sequence, 19), 62);
    assert_eq!(nth_recaman_memoized(&mut sequence, 20), 42);
    assert_eq!(nth_recaman_memoized(&mut sequence, 21), 63);
    assert_eq!(nth_recaman_memoized(&mut sequence, 22), 41);
    assert_eq!(nth_recaman_memoized(&mut sequence, 23), 18);
    assert_eq!(nth_recaman_memoized(&mut sequence, 24), 42);
    assert_eq!(nth_recaman_memoized(&mut sequence, 25), 17);
    assert_eq!(nth_recaman_memoized(&mut sequence, 26), 43);
    assert_eq!(nth_recaman_memoized(&mut sequence, 27), 16);
    assert_eq!(nth_recaman_memoized(&mut sequence, 28), 44);
    assert_eq!(nth_recaman_memoized(&mut sequence, 29), 15);
}

#[test]
fn should_generate_recaman_sequence() {
    let sequence = recaman_sequence(10);
    assert_eq!(sequence, vec![0, 1, 3, 6, 2, 7, 13, 20, 12, 21]);
}
