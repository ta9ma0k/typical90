fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        mut elevations: [i64; n],
        queries: [(usize, usize, i64); q],
    }
    queries.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, (start, end, value))| {
            if i == 0 {
                acc.push(crustal_deformation(elevations.clone(), *start, *end, *value));
            } else {
                acc.push(crustal_deformation(acc[i - 1].clone(), *start, *end, *value));
            }
            acc
        })
        .iter()
        .map(|x| get_inconvenience(x))
        .for_each(|x| println!("{}", x));
}

fn crustal_deformation(elevations: Vec<i64>, start: usize, end: usize, value: i64) -> Vec<i64> {
    elevations.iter().enumerate().map(|(i, x)| match i {
        i if i >= start - 1 && i <= end - 1 => x + value,
        _ => *x,
    }).collect()
}

fn get_inconvenience(elevations: &Vec<i64>) -> i64 {
    (1..elevations.len()).fold(0, |acc, x| {
        acc + (elevations[x - 1] - elevations[x]).abs()
    })
}

#[test]
fn test_get_inconvenience() {
    assert_eq!(get_inconvenience(&vec![1,3,4]), 3);
    assert_eq!(get_inconvenience(&vec![0,2,4]), 4);
}

#[test]
fn test_crustal_deformation() {
    assert_eq!(crustal_deformation(vec![1,2,3], 2, 3, 1), vec![1,3,4]);
    assert_eq!(crustal_deformation(vec![1,3,4], 1, 2, -1), vec![0,2,4]);
}
