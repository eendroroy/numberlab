#[macro_export]
macro_rules! big_u_vec {
    ($($x:expr),+ $(,)?) => {
        vec![$(BigUint::from($x)),+]
    };
}

#[macro_export]
macro_rules! big_u {
    ($x:expr) => {
        BigUint::from($x)
    };
}
