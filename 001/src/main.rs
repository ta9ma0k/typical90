use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, l): (usize, u128),
        k: usize,
        a: [u128; n],
    }
    println!("{}", resolve(a, l, k).unwrap());
}

fn resolve(a: Vec<u128>, l: u128, k: usize) -> Option<u128> {
    cut_combinations(a, k, l)
        .into_iter()
        .map(|v| pice_length_vec(v))
        .flat_map(|v| v.into_iter().min())
        .max()
}

#[test]
fn test_resolve() {
    assert_eq!(resolve(vec![8, 13, 26], 34, 1), Some(13));
}

fn pice_length_vec(a: Vec<u128>) -> Vec<u128> {
    a.iter().tuple_windows().map(|(x, y)| y - x).collect_vec()
}

#[test]
fn test_pice_length_vec() {
    assert_eq!(pice_length_vec(vec![0, 8, 34]), vec![8, 26]);
    assert_eq!(pice_length_vec(vec![0, 13, 34]), vec![13, 21]);
}

fn cut_combinations(a: Vec<u128>, k: usize, l: u128) -> Vec<Vec<u128>> {
    a.into_iter()
        .combinations(k)
        .map(|mut v| {
            v.insert(0, 0);
            v.push(l);
            v
        })
        .collect_vec()
}

#[test]
fn test_cut_combinations() {
    assert_eq!(
        cut_combinations(vec![8, 13, 26], 1, 34),
        vec![vec![0, 8, 34], vec![0, 13, 34], vec![0, 26, 34]]
    );
}
