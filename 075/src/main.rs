fn main() {
    proconio::input! {
        n: usize,
    }
    let answer = (prime_factorization(n).len() as f64).log2().ceil();
    println!("{}", answer as usize);
}

fn prime_factorization(mut n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            res.push(i);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.push(n);
    }
    res
}

#[test]
fn test_prime_factorization() {
    assert_eq!(prime_factorization(2), vec![2]);
    assert_eq!(prime_factorization(36), vec![2, 2, 3, 3]);
    assert_eq!(prime_factorization(42), vec![2, 3, 7]);
}
