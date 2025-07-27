// first approach: use an incremental sieve of eratosthenes
// this will have O(n) space complexity
// if that becomes a problem (which it shouldn't with numbers this low)
// we move on to segmented sieving
fn incremental_sieve_of_e(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }

    let mut primes: Vec<u64> = vec![2];
    let mut candidate: u64 = 3;
    let mut is_prime: bool;

    while primes.len() < n as usize {
        is_prime = true;

        for prime in &primes {
            if prime * prime > candidate {
                break; // No need to check beyond the sqare root
            }
            if candidate % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(candidate);
        }

        candidate += 2; //only check uneven numbers
    }

    return Some(primes[(n - 1) as usize]);
}

fn main() {
    let n: u64 = 10001;

    let res = incremental_sieve_of_e(n);
    match res {
        Some(prime) => {
            println!("the {n}th prime is {prime}");
        }
        None => {
            println!("something went wrong");
        }
    }
}
