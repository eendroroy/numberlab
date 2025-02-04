use numberlab::formula::permutation::permutation;

#[test]
fn should_generate_combination() {
    assert_eq!(permutation(1, 1), 1);
    assert_eq!(permutation(2, 1), 2);
    assert_eq!(permutation(2, 2), 2);
    assert_eq!(permutation(3, 1), 3);
    assert_eq!(permutation(3, 2), 6);
    assert_eq!(permutation(3, 3), 6);
    assert_eq!(permutation(4, 1), 4);
    assert_eq!(permutation(4, 2), 12);
    assert_eq!(permutation(4, 3), 24);
    assert_eq!(permutation(4, 4), 24);
    assert_eq!(permutation(5, 1), 5);
    assert_eq!(permutation(5, 2), 20);
    assert_eq!(permutation(5, 3), 60);
    assert_eq!(permutation(5, 4), 120);
    assert_eq!(permutation(5, 5), 120);
}
