fn main() {
    proconio::input! {
        n: usize,
    }
    if n % 2 == 1 {
        return;
    }
    get_combinations(n)
        .iter()
        .filter(|s| is_valid(s))
        .for_each(|s| println!("{}", s));
}

fn is_valid(s: &str) -> bool {
    s.chars().fold(0, |acc, c| {
        if acc < 0 {
            return acc;
        }
        match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        }
    }) == 0
}

fn get_combinations(n: usize) -> Vec<String> {
    let mut result = (0..(1 << n))
        .map(|bit| {
            (0..n)
                .map(|i| if bit & (1 << i) > 0 { '(' } else { ')' })
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    result.sort();
    result
}

#[test]
fn test_get_combinations() {
    assert_eq!(get_combinations(2), vec!["((", "()", ")(", "))"]);
}

#[test]
fn test_is_valid() {
    assert!(is_valid("()"));
    assert!(is_valid("()()"));
    assert!(is_valid("(())"));

    assert!(!is_valid(")("));
    assert!(!is_valid("())("));
}
