use itertools::iproduct;
fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let m = |x: Vec<usize>| {
        x.into_iter().fold(vec![0i128; 46], |mut acc, x| {
            acc[x % 46] += 1;
            acc
        })
    };
    let ma = m(a);
    let mb = m(b);
    let mc = m(c);
    let result: i128 = iproduct!(0..46, 0..46, 0..46)
        .filter(|&(i, j, k)| (i + j + k) % 46 == 0)
        .map(|(i, j, k)| ma[i] * mb[j] * mc[k])
        .sum();
    println!("{}", result);
}
