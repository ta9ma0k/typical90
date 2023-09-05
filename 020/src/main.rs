fn main() {
    proconio::input! {
        a: i128,
        b: u32,
        c: i128,
    }
    println!("{}", if a < c.pow(b) { "Yes" } else { "No" });
}
