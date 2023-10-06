use proconio::input;

fn main() {
    input! {
        (h, w): (u32, u32),
    }
    println!("{}", resolve(h, w));
}

fn resolve(h: u32, w: u32) -> u32 {
    match (h, w) {
        (1, _) | (_, 1) => h * w,
        _ => ((h + 1) / 2) * ((w + 1) / 2),
    }
}

#[test]
fn test_resolve() {
    assert_eq!(resolve(2, 3), 2);
    assert_eq!(resolve(3, 4), 4);
    assert_eq!(resolve(3, 6), 6);
    assert_eq!(resolve(1, 8), 8);
}
