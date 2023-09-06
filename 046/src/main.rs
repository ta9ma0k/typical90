fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n],
    }
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if (a[i] + b[j] + c[k]) % 46 == 0 {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
