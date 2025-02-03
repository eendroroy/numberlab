use numberlab::sequence::geometric::{geometric_sequence, geometric_series, nth_geometric};

fn round_to_precision(number: f64, precision: u8) -> f64 {
    (number * 10f64.powf(precision as f64)).round() / 10f64.powf(precision as f64)
}

#[test]
#[should_panic(expected = "'n' must be greater than 0")]
fn nth_geometric_should_panic_when_n_is_less_than_1() {
    nth_geometric(1.0, 1.0, 0);
}

#[test]
#[should_panic(expected = "'n' must be greater than 0")]
fn geometric_sequence_should_panic_when_n_is_less_than_1() {
    geometric_sequence(1.0, 1.0, 0);
}

#[test]
fn should_calculate_nth_geometric() {
    assert_eq!(round_to_precision(nth_geometric(1.0, 1.2, 1), 5), 1.0);
    assert_eq!(round_to_precision(nth_geometric(1.0, 1.2, 2), 5), 1.2);
    assert_eq!(round_to_precision(nth_geometric(1.0, 1.2, 3), 5), 1.44);
    assert_eq!(round_to_precision(nth_geometric(1.0, 1.2, 4), 5), 1.728);
    assert_eq!(round_to_precision(nth_geometric(1.0, 1.2, 5), 5), 2.0736);
}

#[test]
fn should_generate_geometric_sequence() {
    let sequence = geometric_sequence(1.0, 1.2, 1);
    assert_eq!(sequence, vec![1.0, ]);

    let sequence = geometric_sequence(1.0, 1.2, 5)
        .iter()
        .map(|n| round_to_precision(*n, 5))
        .collect::<Vec<f64>>();
    assert_eq!(sequence, vec![1.0, 1.2, 1.44, 1.728, 2.0736]);

    let sequence = geometric_sequence(1.0, 1.3, 5)
        .iter()
        .map(|n| round_to_precision(*n, 5))
        .collect::<Vec<f64>>();
    assert_eq!(sequence, vec![1.0, 1.3, 1.69, 2.197, 2.8561]);
}

#[test]
fn should_calculate_geometric_series() {
    assert_eq!(round_to_precision(geometric_series(1.0, 1.0, 0), 6), 0.0);
    assert_eq!(round_to_precision(geometric_series(1.0, 1.0, 1), 6), 1.0);
    assert_eq!(round_to_precision(geometric_series(1.0, 1.0, 2), 6), 2.0);
    assert_eq!(round_to_precision(geometric_series(1.0, 1.0, 3), 6), 3.0);
    assert_eq!(round_to_precision(geometric_series(1.0, 1.0, 4), 6), 4.0);
    assert_eq!(round_to_precision(geometric_series(1.0, 1.3, 5), 6), 9.0431);
    assert_eq!(
        round_to_precision(geometric_series(1.0, 1.3, 6), 6),
        12.75603
    );
    assert_eq!(
        round_to_precision(geometric_series(1.0, 1.3, 7), 6),
        17.582839
    );
}
