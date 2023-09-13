fn main() {
    proconio::input! {
        n: usize,
        a: [[usize; 6]; n]
    }
    println!("{:?}", accumulation(a));
}

fn accumulation(a: Vec<Vec<usize>>) -> usize {
    a.iter().map(|x| x.iter().sum())
        .fold(1, |acc, x: usize| acc * x % 1_000_000_007)
}

#[test]
fn test_accumulation() {
    assert_eq!(
        accumulation(
            vec![
                vec![1,2,3,5,7,11],
                vec![4,6,8,9,10,12]
            ]
        ),
        1421
    );
}
