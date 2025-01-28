use numberlab::prime::sieve::eratosthenes::eratosthenes_sequence;

#[test]
fn should_generate_eratosthenes_sieve_prime_sequence() {
    let n = 1000;
    let primes = eratosthenes_sequence(n);
    assert_eq!(primes.len(), 168);
    assert_eq!(primes[0], 2);
    assert_eq!(primes[1], 3);
    assert_eq!(primes[2], 5);
    assert_eq!(primes[3], 7);
    assert_eq!(primes[4], 11);
    assert_eq!(primes[5], 13);
    assert_eq!(primes[6], 17);
    assert_eq!(primes[7], 19);
    assert_eq!(primes[8], 23);
    assert_eq!(primes[9], 29);
    assert_eq!(primes[10], 31);
    assert_eq!(primes[11], 37);
    assert_eq!(primes[12], 41);
    assert_eq!(primes[13], 43);
    assert_eq!(primes[14], 47);
    assert_eq!(primes[15], 53);
    assert_eq!(primes[16], 59);
    assert_eq!(primes[17], 61);
    assert_eq!(primes[18], 67);
    assert_eq!(primes[19], 71);
    assert_eq!(primes[20], 73);
    assert_eq!(primes[21], 79);
    assert_eq!(primes[22], 83);
    assert_eq!(primes[23], 89);
    assert_eq!(primes[24], 97);
    assert_eq!(primes[25], 101);
    assert_eq!(primes[26], 103);
    assert_eq!(primes[27], 107);
    assert_eq!(primes[28], 109);
    assert_eq!(primes[29], 113);
    assert_eq!(primes[30], 127);
    assert_eq!(primes[31], 131);
    assert_eq!(primes[32], 137);
    assert_eq!(primes[33], 139);
    assert_eq!(primes[34], 149);
    assert_eq!(primes[35], 151);
    assert_eq!(primes[36], 157);
}
