fn main() {
    proconio::input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        b: [i32; q],
    }

    a.sort();
    b.iter()
        .map(|key| get_min_cost(&a, key))
        .for_each(|cost| println!("{}", cost));
}

fn get_min_cost(a: &[i32], key: &i32) -> i32 {
    match a.binary_search(key) {
        Ok(_) => 0,
        Err(i) => {
            let mut min = std::i32::MAX;
            if i < a.len() {
                min = min.min((a[i] - key).abs());
            }
            if i > 0 {
                min = min.min((a[i - 1] - key).abs());
            }
            min
        }
    }
}

#[test]
fn test_get_min_cost() {
    assert_eq!(get_min_cost(&[3200, 4000, 4400, 5000], &3312), 112);
    assert_eq!(get_min_cost(&[3200, 4000, 4400, 5000], &2992), 208);
    assert_eq!(get_min_cost(&[3200, 4000, 4400, 5000], &2992), 208);
    assert_eq!(
        get_min_cost(
            &[
                869120000, 998244353, 777777777, 123456789, 100100100, 464646464, 987654321,
                252525252, 869120001, 1000000000
            ],
            &869120001
        ),
        0
    );
}
