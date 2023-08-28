fn main() {
    proconio::input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
    }

    a.sort();
    b.sort();

    let result = vec![0..n].iter()
        .fold(0, |acc, i| acc + (a[i] - b[i]).abs());

    println!("{}", result);
}
