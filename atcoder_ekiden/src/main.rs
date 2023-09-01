use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(i32, i32); m],
    }
    let combinations = get_combinations(n, &xy);
    let total_time_array = get_total_time_array(&a, &combinations);
    match total_time_array.iter().min() {
        Some(min) => println!("{}", min),
        None => println!("-1"),
    }
}

fn get_total_time_array(a: &Vec<Vec<i32>>, combinations: &Vec<Vec<usize>>) -> Vec<i32> {
    combinations
        .iter()
        .map(|perm| perm.iter().enumerate().map(|(i, v)| (v - 1, i)))
        .map(|perm| perm.map(|(x, n)| a[x][n]).sum())
        .collect_vec()
}

fn get_combinations(n: usize, e_pairs: &Vec<(i32, i32)>) -> Vec<Vec<usize>> {
    (1..(n + 1))
        .permutations(n)
        .filter(|perm| not_exists_error_pair(perm, e_pairs))
        .collect()
}

fn not_exists_error_pair(combinations: &Vec<usize>, e_pairs: &Vec<(i32, i32)>) -> bool {
    (1..combinations.len()).fold(true, |acc, i| {
        acc && !e_pairs.contains(&(combinations[i - 1] as i32, combinations[i] as i32))
            && !e_pairs.contains(&(combinations[i] as i32, combinations[i - 1] as i32))
    })
}

#[test]
fn test_not_exists_pair() {
    assert!(!not_exists_error_pair(&vec![1, 2, 3], &vec![(1, 2)]));
    assert!(!not_exists_error_pair(&vec![1, 2, 3], &vec![(1, 2), (1, 3)]));
    assert!(!not_exists_error_pair(&vec![2, 1, 3], &vec![(1, 2)]));
    assert!(not_exists_error_pair(&vec![1, 2, 3], &vec![(1, 3)]));
}

#[test]
fn test_get_combinations() {
    assert_eq!(
        get_combinations(3, &vec![(1, 2)]),
        vec![vec![1, 3, 2], vec![2, 3, 1]]
    );
    assert!(get_combinations(4, &vec![(1, 2), (1, 3), (2, 3)]).is_empty())
}

#[test]
fn test_get_total_array() {
    assert_eq!(
        get_total_time_array(
            &vec![vec![1, 10, 100], vec![10, 1, 100], vec![100, 10, 1]],
            &vec![vec![1, 3, 2], vec![2, 3, 1]]
        ),
        vec![111, 120]
    )
}
