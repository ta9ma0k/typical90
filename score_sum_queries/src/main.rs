fn main() {
    proconio::input! {
        n: usize,
        a: [[i32; 2]; n],
        q: usize,
        b: [[i32; 2]; q],
    }
    print_vec(&resolve(&a, &b));
}

fn print_vec(list: &Vec<Vec<i32>>) {
    list.iter().for_each(|row| {
        row.iter().for_each(|col| {
            print!("{} ", col);
        });
        println!();
    });
}

fn resolve(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    b.iter()
        .map(|x| {
            let start = x[0] as usize;
            let end = x[1] as usize;
            let list = split_list(&a, &start, &end);
            score_sum(&list)
        })
        .collect()
}

fn split_list(scores: &Vec<Vec<i32>>, start: &usize, end: &usize) -> Vec<Vec<i32>> {
    let start_index = start - 1;
    let end_index = end - 1;
    scores
        .iter()
        .enumerate()
        .filter(|(i, _)| &start_index <= i && i <= &end_index)
        .map(|(_, x)| x.to_vec())
        .collect()
}

fn score_sum(list: &Vec<Vec<i32>>) -> Vec<i32> {
    list.iter().fold(vec![0; 2], |mut acc, x| {
        let index = if x[0] == 1 { 0 } else { 1 };
        acc[index] += x[1];
        acc
    })
}

#[test]
fn test_score_sum() {
    assert_eq!(
        score_sum(&vec![vec![1, 2], vec![2, 3], vec![1, 4]]),
        vec![6, 3]
    );
}

#[test]
fn test_split_list() {
    assert_eq!(
        split_list(
            &vec![vec![1, 2], vec![2, 3], vec![1, 4], vec![2, 5], vec![1, 6]],
            &2,
            &4
        ),
        vec![vec![2, 3], vec![1, 4], vec![2, 5]]
    );
    assert_eq!(
        split_list(
            &vec![vec![1, 2], vec![2, 3], vec![1, 4], vec![2, 5], vec![1, 6]],
            &1,
            &2
        ),
        vec![vec![1, 2], vec![2, 3]]
    )
}

#[test]
fn test_resolve() {
    assert_eq!(
        resolve(
            &vec![
                vec![1, 72],
                vec![2, 78],
                vec![2, 94],
                vec![1, 23],
                vec![2, 89],
                vec![1, 40],
                vec![1, 75],
            ],
            &vec![vec![2, 6]]
        ),
        vec![vec![63, 261]]
    )
}
