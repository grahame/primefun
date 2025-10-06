use std::thread;

use num_prime::nt_funcs::{factorize, is_prime};
use num_prime::{PrimalityTestConfig, RandPrime};

fn cunningham_kind1(start: u128) -> u8 {
    let mut n: u8 = 1;
    let mut x: u128 = start;

    loop {
        x = (x << 1) + 1;
        let p = is_prime(&x, Some(PrimalityTestConfig::strict())).probably();
        if !p {
            break;
        }
        let c = factorize(x);
        if c.len() > 1 {
            break;
        }
        n += 1;
    }

    return n;
}

fn main() {
    let best = 2759832934171386593519u128;

    let num_threads = 8;
    let mut handles = Vec::new();
    for _i in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                let p1: u128 = rng.gen_prime(75, None);
                if p1 > best {
                    let c = cunningham_kind1(p1);
                    if c > 6 {
                        println!("{} {}", p1, c);
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
