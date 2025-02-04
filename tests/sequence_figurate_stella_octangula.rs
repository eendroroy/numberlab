use numberlab::sequence::figurate::stella_octangula::{
    nth_stella_octangula, stella_octangula_sequence,
};

#[test]
fn should_generate_nth_stella_octangula() {
    assert_eq!(nth_stella_octangula(0), 0);
    assert_eq!(nth_stella_octangula(1), 1);
    assert_eq!(nth_stella_octangula(2), 14);
    assert_eq!(nth_stella_octangula(3), 51);
    assert_eq!(nth_stella_octangula(4), 124);
    assert_eq!(nth_stella_octangula(5), 245);
    assert_eq!(nth_stella_octangula(6), 426);
    assert_eq!(nth_stella_octangula(7), 679);
    assert_eq!(nth_stella_octangula(8), 1016);
    assert_eq!(nth_stella_octangula(9), 1449);
}

#[test]
fn should_generate_sequence() {
    assert_eq!(stella_octangula_sequence(0), vec![]);
    assert_eq!(stella_octangula_sequence(1), vec![0]);
    assert_eq!(stella_octangula_sequence(2), vec![0, 1]);
    assert_eq!(
        stella_octangula_sequence(20),
        (0..20)
            .map(|n| nth_stella_octangula(n))
            .collect::<Vec<_>>()
    );
}
