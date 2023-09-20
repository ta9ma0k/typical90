use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    if resolve(a) {
        print!("Yes");
    } else {
        print!("No");
    }
}

fn divided_by_ten(a: &Vec<usize>) -> Option<usize> {
    if a.iter().sum::<usize>() % 10 != 0 {
        return None;
    }
    Some(a.iter().sum::<usize>() / 10)
}

fn resolve(a: Vec<usize>) -> bool {
    let base = match divided_by_ten(&a) {
        Some(x) => x,
        None => return false,
    };
    let cumsum = cumulative_sum(add_zero(add_duplication(a)));
    for i in cumsum.iter() {
        if cumsum.binary_search(&(i + base)).is_ok() {
            return true;
        }
    }
    false
}

fn add_zero(mut a: Vec<usize>) -> Vec<usize> {
    a.insert(0, 0);
    a
}

fn add_duplication(mut a: Vec<usize>) -> Vec<usize> {
    let size = a.len() - 1;
    a.extend(&a.clone()[..size]);
    a
}

fn cumulative_sum(a: Vec<usize>) -> Vec<usize> {
    a.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

#[test]
fn test_base() {
    assert_eq!(divided_by_ten(&vec![1; 10]), Some(1));
    assert_eq!(divided_by_ten(&vec![1; 3]), None);
}

#[test]
fn test_add_zero() {
    assert_eq!(add_zero(vec![1]), vec![0, 1]);
    assert_eq!(add_zero(vec![1, 2]), vec![0, 1, 2]);
}

#[test]
fn test_add_duplication() {
    assert_eq!(add_duplication(vec![1]), vec![1]);
    assert_eq!(add_duplication(vec![1, 2]), vec![1, 2, 1]);
}

#[test]
fn test_cumulative_sum() {
    assert_eq!(cumulative_sum(vec![1, 2, 3]), vec![1, 3, 6]);
    assert_eq!(cumulative_sum(vec![1, 18, 1]), vec![1, 19, 20]);
    assert_eq!(cumulative_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
}

#[test]
fn test_resolve() {
    assert!(resolve(vec![1; 10]));
    assert!(!resolve(vec![1; 3]));
    assert!(resolve(vec![1, 18, 1]));
    assert!(!resolve(vec![1, 9, 1, 9]));
}
