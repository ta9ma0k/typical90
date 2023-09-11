fn main() {
    proconio::input! {
        n: u128,
        l: u128
    }
    let result = (0..=n/l).fold(0, |acc, i| acc + combination(get_steps(n, l, i), i));
    println!("{}", result % 1_000_000_007);
}

fn get_steps(n: u128, l: u128, l_step_count: u128) -> u128 {
    n - (l * l_step_count) + l_step_count
}

fn combination(n: u128, r: u128) -> u128 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

#[test]
fn test_get_steps() {
    assert_eq!(get_steps(3, 2, 1), 2);
    assert_eq!(get_steps(3, 2, 0), 3);
}
