use numberlab::sequence::figurate::hexagonal::{hexagonal_sequence, nth_hexagonal};

#[test]
fn should_generate_nth_hexagonal() {
    assert_eq!(nth_hexagonal(1), (1u128));
    assert_eq!(nth_hexagonal(2), (6u128));
    assert_eq!(nth_hexagonal(3), (15u128));
    assert_eq!(nth_hexagonal(4), (28u128));
    assert_eq!(nth_hexagonal(5), (45u128));
    assert_eq!(nth_hexagonal(6), (66u128));
    assert_eq!(nth_hexagonal(7), (91u128));
    assert_eq!(nth_hexagonal(8), (120u128));
    assert_eq!(nth_hexagonal(9), (153u128));
    assert_eq!(nth_hexagonal(10), (190u128));
}

#[test]
fn should_generate_sequence() {
    assert_eq!(hexagonal_sequence(1), vec![0]);
    assert_eq!(hexagonal_sequence(2), vec![0, 1]);
    assert_eq!(
        hexagonal_sequence(20),
        (0..20).map(|n| nth_hexagonal(n)).collect::<Vec<_>>()
    );
}
