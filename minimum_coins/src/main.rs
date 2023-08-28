fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut ans = 10000;
    for i in 0..=9999 {
        for j in 0..=9999 {
            let tmp = a * i + b*j;
            if tmp > n {
                continue;
            }
            let k = n - tmp;
            if k % c != 0 {
                continue;
            }
            ans = ans.min(i + j + k / c);
        }
    }
    println!("{}", ans);
}
