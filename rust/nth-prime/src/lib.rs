pub fn nth(n: u32) -> u32 {
    let mut counter = 2;
    let mut prime_count = 0;
    let mut last_prime = 2;

    while prime_count != n {
        counter += 1;

        if is_prime(counter) {
            prime_count += 1;
            last_prime = counter;
        }

        if prime_count == n {
            break;
        }
    }

    last_prime
}

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }

        i = i + 6;
    }

    true
}
