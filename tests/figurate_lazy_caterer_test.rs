use numberlab::figurate::lazy_caterer::{lazy_caterer_sequence, nth_lazy_caterer};

#[test]
fn should_generate_nth_lazy_caterer() {
    assert_eq!(nth_lazy_caterer(0), 1);
    assert_eq!(nth_lazy_caterer(1), 2);
    assert_eq!(nth_lazy_caterer(2), 4);
    assert_eq!(nth_lazy_caterer(3), 7);
    assert_eq!(nth_lazy_caterer(4), 11);
    assert_eq!(nth_lazy_caterer(5), 16);
    assert_eq!(nth_lazy_caterer(6), 22);
    assert_eq!(nth_lazy_caterer(7), 29);
    assert_eq!(nth_lazy_caterer(8), 37);
    assert_eq!(nth_lazy_caterer(9), 46);
    assert_eq!(nth_lazy_caterer(10), 56);
}

#[test]
fn should_generate_sequence() {
    assert_eq!(lazy_caterer_sequence(0), vec![]);
    assert_eq!(lazy_caterer_sequence(1), vec![1]);
    assert_eq!(
        lazy_caterer_sequence(20),
        (0..20).map(|n| nth_lazy_caterer(n)).collect::<Vec<_>>()
    );
}
