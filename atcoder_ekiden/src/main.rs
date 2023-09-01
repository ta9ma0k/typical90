use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(i32, i32); m],
    }
    let indexed_xy = xy.iter().map(|v| to_index(*v)).collect_vec();
    let combinations = get_combinations_whitout_error_pairs(n, &indexed_xy);
    let total_time_array = get_total_time_array(&a, &combinations);
    match total_time_array.iter().min() {
        Some(min) => println!("{}", min),
        None => println!("-1"),
    }
}

fn get_total_time_array(a: &Vec<Vec<i32>>, combinations: &Vec<Vec<usize>>) -> Vec<i32> {
    combinations
        .iter()
        .map(|perm| perm.iter().enumerate().map(|(i, j)| a[*j][i]).sum())
        .collect_vec()
}

fn get_combinations_whitout_error_pairs(
    n: usize,
    e_pairs: &Vec<(usize, usize)>,
) -> Vec<Vec<usize>> {
    (0..(n))
        .permutations(n)
        .filter(|perm| not_exists_error_pair(perm, e_pairs))
        .collect()
}

fn not_exists_error_pair(combination: &Vec<usize>, e_pairs: &Vec<(usize, usize)>) -> bool {
    (1..combination.len()).fold(true, |acc, i| {
        acc && !e_pairs.contains(&(combination[i - 1], combination[i]))
            && !e_pairs.contains(&(combination[i], combination[i - 1]))
    })
}

fn to_index(v: (i32, i32)) -> (usize, usize) {
    ((v.0 - 1) as usize, (v.1 - 1) as usize)
}

#[test]
fn test_not_exists_pair() {
    assert!(!not_exists_error_pair(&vec![1, 2, 3], &vec![(1, 2)]));
    assert!(!not_exists_error_pair(
        &vec![1, 2, 3],
        &vec![(1, 2), (1, 3)]
    ));
    assert!(!not_exists_error_pair(&vec![2, 1, 3], &vec![(1, 2)]));
    assert!(not_exists_error_pair(&vec![1, 2, 3], &vec![(1, 3)]));
}

#[test]
fn test_get_combinations_without_error_pairs() {
    assert_eq!(
        get_combinations_whitout_error_pairs(3, &vec![(0, 1)]),
        vec![vec![0, 2, 1], vec![1, 2, 0]]
    );
    assert!(get_combinations_whitout_error_pairs(4, &vec![(0, 1), (0, 2), (1, 2)]).is_empty())
}

#[test]
fn test_get_total_array() {
    assert_eq!(
        get_total_time_array(
            &vec![vec![1, 10, 100], vec![10, 1, 100], vec![100, 10, 1]],
            &vec![vec![0, 2, 1], vec![1, 2, 0]]
        ),
        vec![111, 120]
    )
}
