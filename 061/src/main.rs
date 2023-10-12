use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
        t: [(usize, usize); q]
    }
    resolve(&t).iter().for_each(|x| println!("{}", x));
}

fn resolve(t: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut deck: VecDeque<usize> = VecDeque::new();
    let mut res = vec![];
    for (n, k) in t {
        match *n {
            1 => deck.push_front(*k),
            2 => deck.push_back(*k),
            3 => res.push(deck[*k - 1]),
            _ => {}
        }
    }
    res
}

#[test]
fn test_resolve() {
    assert_eq!(
        resolve(&vec![(1, 2), (1, 1), (2, 3), (3, 1), (3, 2), (3, 3)]),
        vec![1, 2, 3]
    );
}
