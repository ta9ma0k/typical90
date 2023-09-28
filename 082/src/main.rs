use proconio::input;

fn main() {
    input! {
        (l, r): (u128, u128),
    }
    println!("{}", solve(l, r));
}

const MOD: u128 = 1000_000_007;
fn solve(l: u128, r: u128) -> u128 {
    let nl = l.to_string().len();
    let nr = r.to_string().len();
    let mut result = 0;
    for i in nl..=nr {
        let m = 10u128.pow(i as u32) - 1;
        let n = 10u128.pow((i - 1) as u32);
        let a = r.min(m);
        let b = l.max(n) - 1;
        result += (sum(a) - sum(b)) * i as u128;
        result %= MOD;
    }
    result
}

#[test]
fn test_solve() {
    assert_eq!(solve(3, 5), 12);
    assert_eq!(solve(98, 100), 694);
    assert_eq!(solve(1001, 869120), 59367733);
}

fn sum(n: u128) -> u128 {
    n * (n + 1) / 2
}

#[test]
fn test_sum() {
    assert_eq!(sum(1), 1);
    assert_eq!(sum(2), 3);
}
