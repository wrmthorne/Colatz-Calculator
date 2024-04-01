use std::collections::HashSet;

fn colatz_new(n: u64, mut t_main: HashSet<u64>) -> (bool, HashSet<u64>) {
    /* Proven:
     * 1) If n is even:
     *   a) If n_prime is in t_main, replace n_prime with n in t_main (even single-operation principle)
     *   b) If n_prime is not in t_main, n_prime is already a full node in T
     * 2) If n is odd:
     *   a) If n_prime is in t_main, n is already a full node in T (odd multiple-operation principle)
     *   b) If n_prime is not in t_main, ...
     * 3) Need to prove case where loop outside of main tree exists
     */

    let mut n_prime = if (n & 1) == 1 { 3 * n + 1 } else { n >> 1 };
    
    // n_prime in t_main or n_prime is in 2^m trunk
    if (n_prime & (n_prime - 1)) == 0 || t_main.contains(&n_prime) {

        // n is even => max 1 operation conducted => replace n_prime with n
        if n & 1 == 0 {
            t_main.remove(&n_prime);
            t_main.insert(n);
        };
        // n is odd => n is already a full node in T => do nothing

    // n is even and n_prime not in t_main => n_prime already a full node in T
    } else if n & 1 == 0 {

    // n is odd and n_prime not in t_main, create new tree
    } else {
        let mut n_curr = n;
        let mut t_working = HashSet::from([n]);

        loop {
            // If non-full n_prime in t_main or n_prime is in 2^m trunk, replace it with t_working
            if (n_prime & (n_prime - 1)) == 0 || t_main.contains(&n_prime) {
                t_main.remove(&n_prime);              
                t_main.extend(&t_working);
                break;

            // n_curr is odd => add n_prime to t_working 
            } else if n_curr & 1 == 1 {
                t_working.insert(n_prime);
            }

            // n_curr is even => parent will be full => do nothing
            n_curr = n_prime;
            n_prime = if (n_prime & 1) == 1 { 3 * n_prime + 1 } else { n_prime >> 1 };
        }
    }

    (false, t_main)
}

fn main() {    
    // t_main is the set nodes without even children in main tree T
    // Nodes without even children are hence described as non-full
    let mut n: u64 = 2;
    let mut t_main = HashSet::new();
    let mut counter_example: bool = false;

    while !counter_example && n < 10000000 {
        // Can safely skip 1 and 2 as they are trivial. Start with n=3
        n += 1;

        // If n is part of the 2^m trunk, skip n
        if (n & (n - 1)) == 0 {
            continue;
        }

        print!("\rTesting: {}", n);
        (counter_example, t_main) = colatz_new(n, t_main);  
    }

    println!("\n\nColatz counter example found: {}", n);
    println!("\t_main size: {}", t_main.len());
}
