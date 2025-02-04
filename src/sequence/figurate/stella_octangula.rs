pub fn nth_stella_octangula(n: u128) -> u128 {
    match n {
        0 => 0,
        _ => n * (2 * n * n - 1),
    }
}

pub fn stella_octangula_sequence(n: usize) -> Vec<u128> {
    match n {
        0 => vec![],
        _ => (0..n as u128).map(nth_stella_octangula).collect(),
    }
}
