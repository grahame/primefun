use std::thread;

use num_prime::nt_funcs::{factorize, is_prime};
use num_prime::{PrimalityTestConfig, RandPrime};
use std::time::Instant;

fn cunningham_kind1(start: u128) -> u8 {
    fn seek_back(mut k: u128) -> u128 {
        let config = Some(PrimalityTestConfig::strict());
        loop {
            let k2 = (k - 1) >> 1;
            if !is_prime(&k2, config).probably() {
                break;
            }
            let c = factorize(k2);
            if c.len() > 1 {
                break;
            }
            k = k2;
        }
        return k;
    }

    let mut n: u8 = 1;
    let mut x: u128 = seek_back(start);
    let config = Some(PrimalityTestConfig::strict());

    loop {
        x = (x << 1) + 1;
        let p = is_prime(&x, config).probably();
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
    const BLOCK_SIZE: u32 = 1_000_000;

    let num_threads = 8;
    let mut handles = Vec::new();
    for task_id in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                let now = Instant::now();
                for _i in 0..BLOCK_SIZE {
                    let p1: u128 = rng.gen_prime(75, None);
                    if p1 > best {
                        let c = cunningham_kind1(p1);
                        if c > 6 {
                            println!("{} {}", c, p1);
                        }
                    }
                }
                eprintln!(
                    "rate[{}]: {:.2?}/prime",
                    task_id,
                    now.elapsed() / BLOCK_SIZE
                );
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
