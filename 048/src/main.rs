fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [(u64, u64); n],
    }
    let mut points = to_point_list(a);
    points.sort();
    points.reverse();
    let result = points.iter().take(k).sum::<u64>();
    println!("{}", result);
}

fn to_point_list(a: Vec<(u64, u64)>) -> Vec<u64> {
    a.iter().flat_map(|(x, y)| vec![x - y, *y]).collect()
}

#[test]
fn test_to_point_list() {
    assert_eq!(to_point_list(vec![(4, 3), (9, 5)]), vec![1, 3, 4, 5]);
}
