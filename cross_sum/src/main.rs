fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        list: [[i32; w]; h],
    }
    print_vec(&cross_sum(&list));
}

fn print_vec(list: &Vec<Vec<i32>>) {
    list.iter().for_each(|row| {
        row.iter().for_each(|col| {
            print!("{} ", col);
        });
        println!();
    });
}

fn cross_sum(list: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_sum = list
        .iter()
        .map(|row| row.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    let mut col_sum = vec![0; list[0].len()];

    list.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(j, col)| {
            col_sum[j] += col;
        });
    });

    list.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, col)| row_sum[i] + col_sum[j] - col)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[test]
fn cross_sum_test() {
    assert_eq!(cross_sum(&vec![vec![1; 3]; 3]), vec![vec![5; 3]; 3]);
    assert_eq!(
        cross_sum(&vec![
            vec![3, 1, 4, 1],
            vec![5, 9, 2, 6],
            vec![5, 3, 5, 8],
            vec![9, 7, 9, 3],
        ]),
        vec![
            vec![28, 28, 25, 26],
            vec![39, 33, 40, 34],
            vec![38, 38, 36, 31],
            vec![41, 41, 39, 43],
        ]
    );
    assert_eq!(
        cross_sum(&vec![
            vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84],
            vec![62, 64, 33, 83, 27, 95, 2, 88, 41, 97],
        ]),
        vec![
            vec![627, 629, 598, 648, 592, 660, 567, 653, 606, 662],
            vec![623, 633, 651, 618, 645, 650, 689, 685, 615, 676]
        ]
    );
}
