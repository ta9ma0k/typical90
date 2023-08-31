fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64,
    }
    println!("{}", if a < c ^ b { "Yes" } else { "No" });
}
