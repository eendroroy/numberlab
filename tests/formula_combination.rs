use numberlab::formula::combination::combination;

#[test]
fn should_generate_combination() {
    assert_eq!(combination(1, 1), 1);
    assert_eq!(combination(2, 1), 2);
    assert_eq!(combination(2, 2), 1);
    assert_eq!(combination(3, 1), 3);
    assert_eq!(combination(3, 2), 3);
    assert_eq!(combination(3, 3), 1);
    assert_eq!(combination(4, 1), 4);
    assert_eq!(combination(4, 2), 6);
    assert_eq!(combination(4, 3), 4);
    assert_eq!(combination(4, 4), 1);
    assert_eq!(combination(5, 1), 5);
    assert_eq!(combination(5, 2), 10);
    assert_eq!(combination(5, 3), 10);
    assert_eq!(combination(5, 4), 5);
    assert_eq!(combination(5, 5), 1);
    assert_eq!(combination(6, 1), 6);
}
