use num::integer::gcd;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }
    println!("{}", resolve(a, b, c));
}

fn resolve(a: u64, b: u64, c: u64) -> u64 {
    let line = get_line(a, b, c);
    (a / line - 1) + (b / line - 1) + (c / line - 1)
}

#[test]
fn test_resolve() {
    assert_eq!(resolve(2, 2, 3), 4);
    assert_eq!(resolve(2, 2, 4), 1);
}

fn get_line(a: u64, b: u64, c: u64) -> u64 {
    gcd(gcd(a, b), c)
}

#[test]
fn test_get_line() {
    assert_eq!(get_line(2, 2, 3), 1);
    assert_eq!(get_line(2, 2, 4), 2);
}
