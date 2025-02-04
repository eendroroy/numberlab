use numberlab::formula::factorial;
use numberlab::sequence::factorial::factorial_sequence;

#[test]
fn should_generate_factorial_sequence() {
    assert_eq!(factorial_sequence(0), vec![]);
    assert_eq!(factorial_sequence(1), vec![1]);
    assert_eq!(
        factorial_sequence(10),
        (0..10).map(|n| factorial(n)).collect::<Vec<_>>()
    );
}
