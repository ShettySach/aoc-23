use std::iter::zip;

fn main() {
    let lines = std::fs::read_to_string("src/inputs.txt").unwrap();
    let lines: Vec<String> = lines.lines().map(|s| s.to_string()).collect();

    let times: Vec<f64> = lines[0][10..]
        .split_whitespace()
        .map(|s| s.parse().expect("Error in parsing"))
        .collect();

    let dists: Vec<f64> = lines[1][10..]
        .split_whitespace()
        .map(|s| s.parse().expect("Error in parsing"))
        .collect();

    let mut posb: Vec<usize> = Vec::new();

    zip(times, dists).for_each(|(time, dist)| {
        let x1: f64 = (time + f64::sqrt(f64::powi(time, 2) - 4.0 * dist)) / 2.0;
        let x2: f64 = (time - f64::sqrt(f64::powi(time, 2) - 4.0 * dist)) / 2.0;

        let x1 = if x1 > 0.0 {
            f64::floor(x1) as usize
        } else {
            f64::ceil(x1) as usize
        };

        let x2 = if x2 > 0.0 {
            f64::floor(x2) as usize
        } else {
            f64::ceil(x2) as usize
        };

        posb.push(x1 - x2);
    });

    let res: usize = posb.iter().product();
    println!("{}", res);
}
