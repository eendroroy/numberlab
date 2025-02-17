pub trait One {
    fn zero() -> Self;
    fn one() -> Self;
    fn neg_one() -> Self;
}

macro_rules! one_impl {
    ($t:ty, $z:expr, $p:expr, $n:expr) => {
        impl One for $t {
            #[inline(always)]
            fn zero() -> $t {
                $z
            }

            #[inline(always)]
            fn one() -> $t {
                $p
            }

            #[inline(always)]
            fn neg_one() -> $t {
                $n
            }
        }
    };
}

one_impl! { isize, 0_isize, 1_isize, -1_isize }
one_impl! { i8,    0_i8,    1_i8,    -1_i8 }
one_impl! { i16,   0_i16,   1_i16,   -1_i16 }
one_impl! { i32,   0_i32,   1_i32,   -1_i32 }
one_impl! { i64,   0_i64,   1_i64,   -1_i64 }
one_impl! { i128,  0_i128,  1_i128,  -1_i128 }
one_impl! { f32,   0_f32,   1_f32,   -1_f32 }
one_impl! { f64,   0_f64,   1_f64,   -1_f64 }

#[cfg(test)]
mod one_tests {
    use super::*;

    #[test]
    fn should_return_zero() {
        assert_eq!(isize::zero(), 0);
        assert_eq!(i8::zero(), 0);
        assert_eq!(i16::zero(), 0);
        assert_eq!(i32::zero(), 0);
        assert_eq!(i64::zero(), 0);
        assert_eq!(i128::zero(), 0);
        assert_eq!(f32::zero(), 0.0);
        assert_eq!(f64::zero(), 0.0);
    }

    #[test]
    fn should_return_one() {
        assert_eq!(isize::one(), 1);
        assert_eq!(i8::one(), 1);
        assert_eq!(i16::one(), 1);
        assert_eq!(i32::one(), 1);
        assert_eq!(i64::one(), 1);
        assert_eq!(i128::one(), 1);
        assert_eq!(f32::one(), 1.0);
        assert_eq!(f64::one(), 1.0);
    }

    #[test]
    fn should_return_neg_one() {
        assert_eq!(isize::neg_one(), -1);
        assert_eq!(i8::neg_one(), -1);
        assert_eq!(i16::neg_one(), -1);
        assert_eq!(i32::neg_one(), -1);
        assert_eq!(i64::neg_one(), -1);
        assert_eq!(i128::neg_one(), -1);
        assert_eq!(f32::neg_one(), -1.0);
        assert_eq!(f64::neg_one(), -1.0);
    }
}
