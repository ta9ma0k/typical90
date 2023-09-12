fn main() {
    proconio::input! {
        n: usize,
        l: usize 
    }
    let answer = recurrence_formula(n, l)[n - 1];
    println!("{:?}", answer);
}

fn recurrence_formula(n: usize, l: usize) -> Vec<usize> {
    (0..n).into_iter()
        .fold(vec![0; n], |mut acc, i| {
            match i {
                i if i < (l - 1) => acc[i] = 1,
                i if i == (l - 1) => acc[i] = 2,
                _ => acc[i] = (acc[i - 1] + acc[i - l]) % 1_000_000_007,
            } 
            acc
        })
}

#[test]
fn test_recurrence_formula() {
    assert_eq!(recurrence_formula(3, 2), vec![1, 2, 3]);
    assert_eq!(recurrence_formula(5, 2), vec![1, 2, 3, 5, 8]);
    assert_eq!(recurrence_formula(5, 3), vec![1, 1, 2, 3, 4]);
}
