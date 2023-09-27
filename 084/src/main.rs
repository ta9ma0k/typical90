use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    let answer = resolve(&s);
    println!("{:?}", answer);
}

fn resolve(s: &str) -> usize {
    run_length_encoding(&s)
        .iter()
        .map(|(_, n)| count_combinations(*n))
        .fold(count_combinations(s.len()), |acc, x| acc - x)
}

#[test]
fn test_resolve() {
    assert_eq!(resolve("ooxo"), 5);
    assert_eq!(resolve("oooo"), 0);
    assert_eq!(resolve("xxoooxx"), 16);
}

fn count_combinations(n: usize) -> usize {
    n * (n + 1) / 2
}

#[test]
fn test_count_combinations() {
    assert_eq!(count_combinations(12), 78);
    assert_eq!(count_combinations(5), 15);
    assert_eq!(count_combinations(4), 10);
}

fn run_length_encoding(s: &str) -> Vec<(char, usize)> {
    let mut res = vec![];
    let mut chars = s.chars();
    let mut c = chars.next().unwrap();
    let mut n = 1;
    for ch in chars {
        if c == ch {
            n += 1;
        } else {
            res.push((c, n));
            c = ch;
            n = 1;
        }
    }
    res.push((c, n));
    res
}

#[test]
fn test_run_length_encoding() {
    assert_eq!(
        run_length_encoding("oooooxxxxooo"),
        vec![('o', 5), ('x', 4), ('o', 3)]
    );
}
