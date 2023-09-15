fn main() {
    proconio::input! {
        n: usize,
        k: usize,
    }
    println!("{}", solve(n, k));
}

const MOD: usize = 1_000_000_007;
fn solve(n: usize, k: usize) -> usize {
    if n < 2 {
        return k % MOD;
    }
    let l = (k - 1).max(0);
    let m = (k - 2).max(0);
    let x = n - 2;
    pow(m, x) * k % MOD * l % MOD
}

fn pow(mut x: usize, mut n: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * x % MOD;
        }
        n >>= 1;
        x = x * x % MOD;
    }
    res
}

#[test]
fn test_solve() {
    assert_eq!(solve(2, 3), 6);
    assert_eq!(solve(10, 2), 0);
    assert_eq!(solve(2021, 617), 53731843);
}

#[test]
fn test_pow() {
    assert_eq!(pow(2, 0), 1);
    assert_eq!(pow(3, 14), 4782969);
}
