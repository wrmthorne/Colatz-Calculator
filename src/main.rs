fn colatz(n: u64, filter: &u64) -> (bool, u64) {
    // Create new filter not update original filter if counter_example found
    let mut n_filter: u64 = filter.clone();
    let mut next_n: u64 = n;

    loop {
        // All values smaller than n have been checked and powers of 2 always go straight to 1-4 loop
        if next_n < n || (next_n & (next_n - 1)) == 0 {
            return (false, n_filter)

        // If next_n == n, a graph cycle has been found
        } else if next_n == n {
            return (true, filter.clone())

        } else {
            n_filter |= next_n;
            next_n = if (next_n & 1) == 1 { 3 * next_n + 1 } else { next_n >> 1 };
        }

        // Case where the sequence never reaches 0 is a version of the halting problem
        // => uncomputable

        // If the value is definitely not in the filter
        // if next_n & n_filter != next_n || next_n & n_filter != n_filter
    }
}

fn main() {
    // Start from 2 as 1 is base case and to save n == 1 check each loop
    let mut n: u64 = 2; 
    let mut filter: u64 = 3; // 2 = 0010 => filter = 0011 = 3
    let mut counter_example: bool = false;

    while !counter_example {

        print!("\rTesting: {}", n);
        (counter_example, filter) = colatz(n, &filter);

        n += 1;
    }

    println!("Colatz counter example found: {}", n);
}
