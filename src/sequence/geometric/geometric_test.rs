use super::*;

fn round_to_precision(number: f64, precision: u8) -> f64 {
    (number * 10f64.powf(precision as f64)).round() / 10f64.powf(precision as f64)
}

#[test]
fn should_calculate_nth_geometric() {
    assert_eq!(round_to_precision(nth_geometric(1f64, 1.2, 0), 5), 1.0);
    assert_eq!(round_to_precision(nth_geometric(1f64, 1.2, 1), 5), 1.2);
    assert_eq!(round_to_precision(nth_geometric(1f64, 1.2, 2), 5), 1.44);
    assert_eq!(round_to_precision(nth_geometric(1f64, 1.2, 3), 5), 1.728);
    assert_eq!(round_to_precision(nth_geometric(1f64, 1.2, 4), 5), 2.0736);
    assert_eq!(round_to_precision(nth_geometric(1f64, 1.2, 5), 5), 2.48832);
}

#[test]
fn should_generate_geometric_sequence() {
    let sequence = geometric_sequence(1f64, 2f64, 10);
    assert_eq!(
        sequence,
        vec![1f64, 2f64, 4f64, 8f64, 16f64, 32f64, 64f64, 128f64, 256f64, 512f64]
    );

    let sequence = geometric_sequence(1f64, -1.1, 10).iter().map(|n| round_to_precision(*n, 2)).collect::<Vec<f64>>();
    assert_eq!(
        sequence,
        vec![1.0, -1.1, 1.21, -1.33, 1.46, -1.61, 1.77, -1.95, 2.14, -2.36]
    );
}

#[test]
fn should_calculate_geometric_sum() {
    assert_eq!(round_to_precision(geometric_sum(1f64, 1f64, 1), 6), 1f64);
    assert_eq!(round_to_precision(geometric_sum(1f64, 1f64, 2), 6), 2f64);
    assert_eq!(round_to_precision(geometric_sum(1f64, 1f64, 3), 6), 3f64);
    assert_eq!(round_to_precision(geometric_sum(1f64, 1f64, 4), 6), 4f64);
    assert_eq!(round_to_precision(geometric_sum(1f64, 1.3f64, 5), 6), 9.0431);
    assert_eq!(round_to_precision(geometric_sum(1f64, 1.3f64, 6), 6), 12.75603);
    assert_eq!(round_to_precision(geometric_sum(1f64, 1.3f64, 7), 6), 17.582839);
}
