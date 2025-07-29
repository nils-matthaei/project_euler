use std::collections::HashMap;

fn collatz_step(n: u64) -> u64 {
    let n_plus_one;
    if n % 2 == 0 {
        n_plus_one = n / 2;
    } else {
        n_plus_one = 3 * n + 1;
    }
    n_plus_one
}

fn main() {
    // seq_len_map
    //     key: starting number n
    //     value: length of collatz sequence starting with n
    //
    //     it is initialized with the pair (1,1)
    //
    //     u64 for both should suffice, since we only look at n's < 1_000_000
    //     we also trust that the sequence length for all sequences is under 2^32
    let mut seq_len_map: HashMap<u64, u64> = HashMap::new();
    seq_len_map.insert(1, 1);

    // loop over all starting number candidates
    for i in 1..1_000_000 {
        let mut collatz_sequence: Vec<u64> = Vec::new();
        let mut n = i;

        loop {
            collatz_sequence.push(n);
            match seq_len_map.get(&n) {
                // we distinguish two cases here to avoid multiple calcutlation
                // of the same collatz sequence and thus reduce runtime

                // if we already calculated the collatz sequence length l for n
                // then we don't need to continue calculating the sequence because
                // we can now calculate the steps needed to get to one for ALL
                // previously calculated elements of the sequence
                //
                // this works because the collatz sequence is agnostic to all previous
                // steps and only cares about the current value to calculate the next one
                Some(&l) => {
                    let seq_len = collatz_sequence.len();
                    for j in 0..seq_len {
                        let element = collatz_sequence[j];
                        if !seq_len_map.contains_key(&element) {
                            // calculate steps from element to end of the sequence
                            let steps: u64 = (seq_len - j) as u64 + l - 1;
                            seq_len_map.insert(element, steps);
                        }
                    }
                    break;
                }
                // here we calculate the next step in the collatz sequence
                None => {
                    n = collatz_step(n);
                }
            }
        }
    }

    let max = seq_len_map.iter().max_by_key(|&(_, value)| value).unwrap();

    println!(
        "The longest Collatz Sequence under one million starts with {}, it takes {} steps to get to 1.",
        max.0, max.1
    );
}
