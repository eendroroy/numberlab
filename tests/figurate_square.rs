use numberlab::figurate::square::{nth_square, square_sequence};

#[test]
fn should_generate_nth_square() {
    assert_eq!(nth_square(0), 0);
    assert_eq!(nth_square(1), 1);
    assert_eq!(nth_square(2), 4);
    assert_eq!(nth_square(3), 9);
    assert_eq!(nth_square(4), 16);
    assert_eq!(nth_square(5), 25);
    assert_eq!(nth_square(6), 36);
    assert_eq!(nth_square(7), 49);
    assert_eq!(nth_square(8), 64);
    assert_eq!(nth_square(9), 81);
    assert_eq!(nth_square(10), 100);
}

#[test]
fn should_generate_sequence() {
    assert_eq!(square_sequence(0), vec![]);
    assert_eq!(square_sequence(1), vec![0]);
    assert_eq!(
        square_sequence(20),
        (0..20).map(|n| nth_square(n)).collect::<Vec<_>>()
    );
}
