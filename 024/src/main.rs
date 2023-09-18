fn main() {
    proconio::input! {
        (n, k): (usize, usize),
        a: [i32; n],
        b: [i32; n],
    }
    let diff = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs() as usize)
        .sum::<usize>();
    if diff > k || (diff - k) % 2 == 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
