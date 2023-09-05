use num::integer;

fn main() {
    proconio::input! {
        x: u128,
        y: u128,
    }
    let res = integer::lcm(x, y);
    if res > 10_u128.pow(18) {
        println!("Large");
    } else {
        println!("{}", res);
    }
}
