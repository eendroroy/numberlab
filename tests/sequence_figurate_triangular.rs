use numberlab::sequence::figurate::triangular::{nth_triangular, triangular_sequence};

#[test]
fn should_generate_nth_triangular() {
    assert_eq!(nth_triangular(0), (0u128));
    assert_eq!(nth_triangular(1), (1u128));
    assert_eq!(nth_triangular(2), (3u128));
    assert_eq!(nth_triangular(3), (6u128));
    assert_eq!(nth_triangular(4), (10u128));
    assert_eq!(nth_triangular(5), (15u128));
    assert_eq!(nth_triangular(6), (21u128));
    assert_eq!(nth_triangular(7), (28u128));
    assert_eq!(nth_triangular(8), (36u128));
    assert_eq!(nth_triangular(9), (45u128));
    assert_eq!(nth_triangular(10), (55u128));
}

#[test]
fn should_generate_sequence_of_1_items() {
    assert_eq!(triangular_sequence(1), vec![(0u128)]);
}

#[test]
fn should_generate_sequence_of_20_items() {
    assert_eq!(
        triangular_sequence(20),
        vec![
            (0u128),
            (1u128),
            (3u128),
            (6u128),
            (10u128),
            (15u128),
            (21u128),
            (28u128),
            (36u128),
            (45u128),
            (55u128),
            (66u128),
            (78u128),
            (91u128),
            (105u128),
            (120u128),
            (136u128),
            (153u128),
            (171u128),
            (190u128)
        ]
    );
}
