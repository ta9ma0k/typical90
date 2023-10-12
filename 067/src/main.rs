use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let answer = (0..k).fold(n, |acc, _| execute(acc));
    println!("{}", answer);
}

fn execute(n: usize) -> usize {
    change_8_to_5(to_base_9(from_base_8(n)))
}

#[test]
fn test_execute() {
    assert_eq!(execute(21), 15);
    assert_eq!(execute(1330), 555);
}

fn from_base_8(mut n: usize) -> usize {
    let mut result = 0;
    let mut base = 1;
    while n > 0 {
        result += (n % 10) * base;
        n /= 10;
        base *= 8;
    }
    result
}

#[test]
fn test_from_base_8() {
    assert_eq!(from_base_8(1), 1);
    assert_eq!(from_base_8(7), 7);
    assert_eq!(from_base_8(10), 8);
    assert_eq!(from_base_8(21), 17);
}

fn to_base_9(n: usize) -> usize {
    let mut result = 0;
    let mut base = 1;
    let mut n = n;
    while n > 0 {
        result += (n % 9) * base;
        n /= 9;
        base *= 10;
    }
    result
}

#[test]
fn test_to_base_9() {
    assert_eq!(to_base_9(1), 1);
    assert_eq!(to_base_9(8), 8);
    assert_eq!(to_base_9(9), 10);
    assert_eq!(to_base_9(17), 18);
}

fn change_8_to_5(n: usize) -> usize {
    let mut result = 0;
    let mut base = 1;
    let mut n = n;
    while n > 0 {
        if n % 10 == 8 {
            result += 5 * base;
        } else {
            result += (n % 10) * base;
        }
        n /= 10;
        base *= 10;
    }
    result
}

#[test]
fn test_change_8_to_5() {
    assert_eq!(change_8_to_5(1), 1);
    assert_eq!(change_8_to_5(8), 5);
    assert_eq!(change_8_to_5(18), 15);
}
