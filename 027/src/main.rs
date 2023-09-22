use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    register(&s).iter().for_each(|x| println!("{}", x + 1));
}

fn register(requests: &Vec<String>) -> Vec<usize> {
    let mut result = vec![];
    let mut member_set: HashSet<&str> = HashSet::new();
    for i in 0..requests.len() {
        if member_set.insert(&requests[i]) {
            result.push(i);
        }
    }
    result
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
