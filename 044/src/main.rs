fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [i32; n],
        mut t: [(i32, i32, i32); q],
    }
    t.reverse();
    let mut results = Vec::new();
    for (x, y, z) in t {
        match x {
            1 => {
                results = q1(results, y, z);
            }
            2 => {
                results = q2(results, n as i32);
            }
            3 => {
                results = q3(results, y);
            }
            _ => unreachable!(),
        }
    }
    results.reverse();
    results
        .iter()
        .for_each(|x| println!("{}", a[*x as usize - 1]));
}

fn q1(a: Vec<i32>, x: i32, y: i32) -> Vec<i32> {
    let mut a = a;
    for i in 0..a.len() {
        if a[i] == x {
            a[i] = y;
        } else if a[i] == y {
            a[i] = x;
        }
    }
    a
}

//aの値を全てデクリメントする
//aの値が-1の場合はnにする
fn q2(a: Vec<i32>, n: i32) -> Vec<i32> {
    a.iter()
        .map(|x| x - 1)
        .map(|x| if x == 0 { n } else { x })
        .collect()
}

fn q3(a: Vec<i32>, x: i32) -> Vec<i32> {
    let mut a = a;
    a.push(x);
    a
}

#[test]
fn test_q1() {
    assert_eq!(q1(vec![1, 2], 1, 2), vec![2, 1]);
    assert_eq!(q1(vec![3], 1, 2), vec![3]);
}

#[test]
fn test_q2() {
    assert_eq!(q2(vec![2, 3], 3), vec![1, 2]);
    assert_eq!(q2(vec![1, 3], 3), vec![3, 2]);
}

#[test]
fn test_q3() {
    assert_eq!(q3(vec![1, 2], 3), vec![1, 2, 3]);
    assert_eq!(q3(vec![0, 3], 2), vec![0, 3, 2]);
}
