fn main() {
    proconio::input! {
        n: usize,
        a: [[i32; 2]; n],
        q: usize,
        b: [[usize; 2]; q],
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

fn resolve(a: &Vec<Vec<i32>>, b: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
    let list = split_list_by_class(&a)
        .iter()
        .map(|x| cumulative_sum(&x))
        .collect::<Vec<Vec<i32>>>();
    b.iter()
        .map(|x| {
            let end = x[1] - 1;
            if x[0] == 1 {
                vec![list[0][end], list[1][end]]
            } else {
                let start = x[0] - 2;
                vec![list[0][end] - list[0][start], list[1][end] - list[1][start]]
            }
        })
        .collect()
}

fn split_list_by_class(list: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    list.iter()
        .enumerate()
        .fold(vec![vec![0; list.len()]; 2], |mut acc, (index, x)| {
            let class_num = if x[0] == 1 { 0 } else { 1 };
            acc[class_num][index] = x[1];
            acc
        })
}


fn cumulative_sum(list: &Vec<i32>) -> Vec<i32> {
    list.iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

#[test]
fn test_split_list_by_class() {
    assert_eq!(
        split_list_by_class(&vec![
            vec![1, 10],
            vec![1, 10],
            vec![2, 10],
            vec![1, 10],
            vec![2, 10]
        ]),
        vec![vec![10, 10, 0, 10, 0], vec![0, 0, 10, 0, 10]]
    );
    assert_eq!(
        split_list_by_class(&vec![vec![1, 10], vec![1, 10], vec![1, 10],]),
        vec![vec![10, 10, 10], vec![0, 0, 0]]
    );
}
#[test]
fn test_cumulative_sum() {
    assert_eq!(
        cumulative_sum(&vec![10, 0, 10, 10, 10]),
        vec![10, 10, 20, 30, 40]
    );
    assert_eq!(
        cumulative_sum(&vec![10, 0, 0, 0, 10]),
        vec![10, 10, 10, 10, 20]
    );
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
    );
    assert_eq!(
        resolve(
            &vec![vec![1, 100], vec![1, 100], vec![1, 100],],
            &vec![vec![1, 3]]
        ),
        vec![vec![300, 0]]
    );
}
