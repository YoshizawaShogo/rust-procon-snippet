use cargo_snippet::snippet;

#[snippet("IS_PRIME")]
pub fn is_prime(num: usize) -> bool {
    let end = (num as f64).sqrt().floor() as usize;
    for i in 2..=end {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[snippet("PRIME_FACTORIZATION")]
pub fn prime_factorization(mut num: usize) -> Vec<usize> {
    let end = (num as f64).sqrt().floor() as usize;
    let mut primes = Vec::new();
    for i in 2..=end {
        while num % i == 0 {
            primes.push(i);
            num /= i;
        }
    }

    if primes.len() == 0 {
        primes.push(num);
    }
    primes
}

#[snippet("PRIMES_UNDER")]
pub fn primes_under(num: usize) -> Vec<usize> {
    let mut may_be_prime_list = vec![true; num + 1];
    may_be_prime_list[0] = false;
    may_be_prime_list[1] = false;

    let end = (num as f64).sqrt().floor() as usize;

    for i in 2..=end {
        if may_be_prime_list[i] {
            let mut target = i + i;
            while target <= num {
                may_be_prime_list[target] = false;
                target += i;
            }
        }
    }
    may_be_prime_list
        .into_iter()
        .enumerate()
        .filter(|&(_, flag)| flag)
        .map(|(index, _)| index)
        .collect()
}
