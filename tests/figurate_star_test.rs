use numberlab::figurate::star::{nth_star, star_sequence};

#[test]
fn should_generate_nth_star() {
    assert_eq!(nth_star(1), 1);
    assert_eq!(nth_star(2), 13);
    assert_eq!(nth_star(3), 37);
    assert_eq!(nth_star(4), 73);
    assert_eq!(nth_star(5), 121);
    assert_eq!(nth_star(6), 181);
    assert_eq!(nth_star(7), 253);
    assert_eq!(nth_star(8), 337);
    assert_eq!(nth_star(9), 433);
    assert_eq!(nth_star(10), 541);
}

#[test]
fn should_generate_sequence() {
    assert_eq!(star_sequence(0), vec![]);
    assert_eq!(star_sequence(1), vec![1]);
    assert_eq!(star_sequence(2), vec![1, 13]);
    assert_eq!(
        star_sequence(20),
        (1..=20).map(|n| nth_star(n)).collect::<Vec<_>>()
    );
}
