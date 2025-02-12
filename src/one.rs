/// A trait that defines a method to return the default value of `1`.
pub trait One {
    /// Returns the default value of `1`.
    fn one() -> Self;
    fn neg_one() -> Self;
}

macro_rules! one_impl {
    ($t:ty, $p:expr, $n:expr, $doc:tt) => {
        impl One for $t {
            #[inline(always)]
            #[doc = $doc]
            fn one() -> $t {
                $p
            }

            #[inline(always)]
            #[doc = $doc]
            fn neg_one() -> $t {
                $n
            }
        }
    };
}

one_impl! { isize, 1, -1, "Returns the default value of `0`" }
one_impl! { i8, 1, -1, "Returns the default value of `0`" }
one_impl! { i16, 1, -1, "Returns the default value of `0`" }
one_impl! { i32, 1, -1, "Returns the default value of `0`" }
one_impl! { i64, 1, -1, "Returns the default value of `0`" }
one_impl! { i128, 1, -1, "Returns the default value of `0`" }

one_impl! { f32, 1.0f32, -1.0f32, "Returns the default value of `0.0`" }
one_impl! { f64, 1.0f64, -1.0f64, "Returns the default value of `0.0`" }
