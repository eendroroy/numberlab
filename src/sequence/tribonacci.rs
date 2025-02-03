use num::BigUint;

/// Calculates the nth Tribonacci number.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Tribonacci sequence.
///
/// # Returns
///
/// The nth Tribonacci number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::tribonacci::nth_tribonacci;
///
/// let result = nth_tribonacci(10);
/// assert_eq!(result, BigUint::from(149u128));
/// ```
pub fn nth_tribonacci(n: usize) -> BigUint {
    match n {
        0 => BigUint::from(0u128),
        1 => BigUint::from(1u128),
        2 => BigUint::from(1u128),
        _ => nth_tribonacci(n - 3) + nth_tribonacci(n - 2) + nth_tribonacci(n - 1),
    }
}

/// Calculates the nth Tribonacci number using memoization.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the position in the Tribonacci sequence.
///
/// # Returns
///
/// The nth Tribonacci number .
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::tribonacci::nth_tribonacci_memoized;
///
/// let result = nth_tribonacci_memoized(10);
/// assert_eq!(result, BigUint::from(149u128));
/// ```
pub fn nth_tribonacci_memoized(n: usize) -> BigUint {
    let mut memoizer = vec![
        BigUint::from(0u128),
        BigUint::from(1u128),
        BigUint::from(1u128),
    ];
    nth_tribonacci_with_memoizer(n, &mut memoizer)
}

fn nth_tribonacci_with_memoizer(n: usize, memoizer: &mut Vec<BigUint>) -> BigUint {
    if memoizer.len() > n {
        memoizer[n].clone()
    } else {
        let kth_tribonacci = nth_tribonacci_with_memoizer(n - 3, memoizer);
        let lth_tribonacci = nth_tribonacci_with_memoizer(n - 2, memoizer);
        let mth_tribonacci = nth_tribonacci_with_memoizer(n - 1, memoizer);
        let nth_tribonacci = kth_tribonacci + lth_tribonacci + mth_tribonacci;
        memoizer.push(nth_tribonacci);
        memoizer[n].clone()
    }
}

/// Generates a sequence of Tribonacci numbers up to the given number `n`.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of terms in the Tribonacci sequence.
///
/// # Returns
///
/// A vector containing the Tribonacci sequence up to the given number `n`.
///
/// # Examples
///
/// ```
/// use num::BigUint;
/// use numberlab::sequence::tribonacci::tribonacci_sequence;
///
/// let sequence = tribonacci_sequence(5);
/// assert_eq!(
///     sequence,
///     vec![
///         BigUint::from(0_u128),
///         BigUint::from(1_u128),
///         BigUint::from(1_u128),
///         BigUint::from(2_u128),
///         BigUint::from(4_u128)
///     ]
/// );
/// ```
pub fn tribonacci_sequence(n: usize) -> Vec<BigUint> {
    match n {
        0 => vec![],
        1 => vec![BigUint::from(0u128)],
        2 => vec![BigUint::from(0u128), BigUint::from(1u128)],
        3 => vec![
            BigUint::from(0u128),
            BigUint::from(1u128),
            BigUint::from(1u128),
        ],
        _ => {
            let mut sequence = vec![
                BigUint::from(0u128),
                BigUint::from(1u128),
                BigUint::from(1u128),
            ];
            (3..n).for_each(|i| {
                sequence.push(&sequence[i - 1] + &sequence[i - 2] + &sequence[i - 3])
            });
            sequence
        }
    }
}
