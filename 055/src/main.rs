use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, p, q): (usize, usize, usize),
        a: [usize; n],
    }
    println!("{}", solve(&a, p, q));
}

fn solve(a: &Vec<usize>, divider: usize, remainder: usize) -> usize {
    (0..a.len())
        .combinations(5)
        .filter(|c| {
            a[c[0]] * a[c[1]] % divider * a[c[2]] % divider * a[c[3]] % divider * a[c[4]] % divider
                == remainder
        })
        .count()
}

#[test]
fn test_solve() {
    assert_eq!(solve(&vec![1, 2, 3, 4, 5, 6], 7, 1), 1);
    assert_eq!(solve(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1, 0), 252);
}
