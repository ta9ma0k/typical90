use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    register(&s).iter().for_each(|x| println!("{}", x + 1));
}

fn register(requests: &Vec<String>) -> Vec<usize> {
    let mut res: Vec<usize> = requests
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, val)| {
            acc.entry(val).or_insert(i);
            acc
        })
        .values()
        .cloned()
        .collect();
    res.sort();
    res
}

#[test]
fn test_register() {
    assert_eq!(
        register(&vec![
            String::from("hoge"),
            String::from("fuga"),
            String::from("hoge")
        ]),
        vec![0, 1]
    );
    assert_eq!(
        register(&vec![
            String::from("hoge"),
            String::from("fuga"),
            String::from("hoge1")
        ]),
        vec![0, 1, 2]
    );
}
