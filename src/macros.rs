#[macro_export]
macro_rules! big_u_vec {
    ($($x:expr),+ $(,)?) => {
        vec![$(BigUint::from($x)),+]
    };
}
