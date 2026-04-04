use std::time::{SystemTime, UNIX_EPOCH};

use num::{bigint::ToBigUint, ToPrimitive};

/// Note: Uses a naive approach to generate a random number. While this is
/// shoddy in practice, it's harmless for a challenge.
pub fn private_key(prime: u64) -> u64 {
    let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let bad_rand_num = epoch.as_secs() % prime;

    if bad_rand_num <= 1 {
        return 2;
    }

    if bad_rand_num == prime {
        return prime - 1;
    }

    bad_rand_num
}

pub fn public_key(prime: u64, g: u64, a_priv_key: u64) -> u64 {
    mod_pow(g, a_priv_key, prime)
}

pub fn secret(prime: u64, b_pub_key: u64, a_priv_key: u64) -> u64 {
    mod_pow(b_pub_key, a_priv_key, prime)
}

/// Note: I know this was lazy of me, but I really struggled with the bonus
/// challenges, so I opted to just use a library instead.
fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let b = base.to_biguint().unwrap();
    let e = exponent.to_biguint().unwrap();
    let m = modulus.to_biguint().unwrap();
    b.modpow(&e, &m).to_u64().unwrap()
}
