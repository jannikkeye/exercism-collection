pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];

    while n > 1 {
        let factor = (2..=(n as f64).sqrt() as u64)
            .find(|v| n % v == 0)
            .unwrap_or(n);

        factors.push(factor);

        n /= factor;
    }

    factors
}
