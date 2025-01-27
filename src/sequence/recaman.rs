pub fn nth_recaman_memoized(memoizer: &mut Vec<u128>, n: usize) -> u128 {
    if memoizer.len() == 0 {
        memoizer.push(0);
    }

    if memoizer.len() > n {
        memoizer[n]
    } else {
        let mth_recaman = nth_recaman_memoized(memoizer, n - 1);
        let nth_recaman =
            if mth_recaman > n as u128 && !memoizer.contains(&(mth_recaman - n as u128)) {
                mth_recaman - n as u128
            } else {
                mth_recaman + n as u128
            };
        memoizer.push(nth_recaman);
        nth_recaman
    }
}

pub fn recaman_sequence(n: usize) -> Vec<u128> {
    let mut sequence = vec![0];
    for i in 2..n {
        nth_recaman_memoized(&mut sequence, i);
    }
    sequence
}