use std::f64::consts::PI;

fn main() {
    proconio::input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e_list: [f64; q],
    }

    let statue_positon = (x, y, 0.);
    e_list
        .iter()
        .map(|e| get_wheel_position(l, *e, t))
        .map(|wheel| {
            let dist = get_xy_distance(statue_positon, wheel);
            let atant = (wheel.2 / dist).atan();
            atant.to_degrees()
        })
        .for_each(|degree| {
            println!("{}", degree);
        });
}

fn get_xy_distance(center: (f64, f64, f64), wheel: (f64, f64, f64)) -> f64 {
    let x = wheel.0 - center.0;
    let y = wheel.1 - center.1;
    (x.powi(2) + y.powi(2)).sqrt()
}

fn get_wheel_position(l: f64, e: f64, t: f64) -> (f64, f64, f64) {
    let theta = e / t * 2.0 * PI - PI / 2.0;
    let r = l / 2.0;
    let y = -1. * r * theta.cos();
    let z = r + r * theta.sin();
    (0.0, y, z)
}
