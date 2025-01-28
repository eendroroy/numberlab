use numberlab::sequence::figurate::square::{nth_square, square_sequence};

#[test]
fn should_generate_nth_square() {
    assert_eq!(nth_square(1), 1u128.into());
    assert_eq!(nth_square(2), 4u128.into());
    assert_eq!(nth_square(3), 9u128.into());
    assert_eq!(nth_square(4), 16u128.into());
    assert_eq!(nth_square(5), 25u128.into());
    assert_eq!(nth_square(6), 36u128.into());
    assert_eq!(nth_square(7), 49u128.into());
    assert_eq!(nth_square(8), 64u128.into());
    assert_eq!(nth_square(9), 81u128.into());
    assert_eq!(nth_square(10), 100u128.into());
}

#[test]
fn should_generate_series_of_1_items() {
    assert_eq!(square_sequence(1), vec![1u128.into()]);
}

#[test]
fn should_generate_series_of_20_items() {
    assert_eq!(
        square_sequence(20),
        vec![
            1u128.into(),
            4u128.into(),
            9u128.into(),
            16u128.into(),
            25u128.into(),
            36u128.into(),
            49u128.into(),
            64u128.into(),
            81u128.into(),
            100u128.into(),
            121u128.into(),
            144u128.into(),
            169u128.into(),
            196u128.into(),
            225u128.into(),
            256u128.into(),
            289u128.into(),
            324u128.into(),
            361u128.into(),
            400u128.into()
        ]
    );
}
