use numberlab::sequence::figurate::pentagonal::{nth_pentagonal, pentagonal_sequence};

#[test]
fn should_generate_nth_pentagonal() {
    assert_eq!(nth_pentagonal(0), 0);
    assert_eq!(nth_pentagonal(1), 1);
    assert_eq!(nth_pentagonal(2), 5);
    assert_eq!(nth_pentagonal(3), 12);
    assert_eq!(nth_pentagonal(4), 22);
    assert_eq!(nth_pentagonal(5), 35);
    assert_eq!(nth_pentagonal(6), 51);
    assert_eq!(nth_pentagonal(7), 70);
    assert_eq!(nth_pentagonal(8), 92);
    assert_eq!(nth_pentagonal(9), 117);
    assert_eq!(nth_pentagonal(10), 145);
}

#[test]
fn should_generate_sequence() {
    assert_eq!(pentagonal_sequence(0), vec![]);
    assert_eq!(pentagonal_sequence(1), vec![0]);
    assert_eq!(
        pentagonal_sequence(20),
        (0..20).map(|n| nth_pentagonal(n)).collect::<Vec<_>>()
    );
}
