fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [i32; n],
        t: [(i32, i32, i32); q],
    }
    t.iter()
        .rev()
        .fold(vec![], |acc, (x, y, z)| match x {
            1 => q1(acc, *y, *z),
            2 => q2(acc, n as i32),
            3 => q3(acc, *y),
            _ => unreachable!(),
        })
        .iter()
        .rev()
        .map(|x| *x as usize - 1)
        .for_each(|x| println!("{}", a[x]));
}

fn q1(a: Vec<i32>, x: i32, y: i32) -> Vec<i32> {
    a.iter()
        .map(|z| match x {
            _ if *z == x => y,
            _ if *z == y => x,
            _ => *z,
        })
        .collect()
}

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
