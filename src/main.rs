use num_prime::nt_funcs::{factorize, is_prime};
use num_prime::{Primality, PrimalityTestConfig};

fn cunningham_kind1(start: u128) -> Option<u8> {
    let mut n: u8 = 1;
    let mut x: u128 = start;

    loop {
        x = (x << 1) + 1;
        let p = is_prime(&x, Some(PrimalityTestConfig::strict())).probably();
        if !p {
            break;
        }
        let c = factorize(x);
        if (c.len() > 1) {
            break;
        }
        n += 1;
    }

    return Some(n);
}

fn main() {
    println!("Hello, world!");
    let x = 2759832934171386593519u128;
    println!("{:?}", cunningham_kind1(x));
}
