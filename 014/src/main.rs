fn main() {
    proconio::input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }

    a.sort();
    b.sort();

    let mut result = 0;
    for i in 0..n {
        result += (a[i] - b[i]).abs();
    }

    println!("{}", result);
}
