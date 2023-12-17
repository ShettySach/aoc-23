fn extrap(y: &Vec<isize>) -> isize {
    let y0 = y[0];
    if y.iter().skip(1).all(|&x| x == y0) {
        return y0;
    }

    let n = y.len();
    let mut yp = vec![0; n - 1];
    for i in 0..n - 1 {
        yp[i] = y[i + 1] - y[i];
    }

    let ypn = extrap(&yp);
    return ypn + y.last().unwrap();
}

fn rev_extrap(y: &Vec<isize>) -> isize {
    let y0 = y[0];
    if y.iter().skip(1).all(|&x| x == y0) {
        return y0;
    }
    let n = y.len();
    let mut yp = vec![0; n - 1];
    let yr: Vec<_> = y.iter().rev().collect();
    for i in 0..n - 1 {
        yp[i] = yr[i + 1] - yr[i];
    }
    let ypn = extrap(&yp);
    return ypn + y.first().unwrap();
}

fn main() {
    let data = include_str!("inputs.txt");
    let data: Vec<&str> = data.lines().collect();
    let mut lines: Vec<Vec<isize>> = Vec::new();

    data.iter().for_each(|line| {
        let line: Vec<isize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        lines.push(line);
    });

    let mut sum_a = 0;
    lines.iter().for_each(|line| {
        sum_a += extrap(line);
    });

    let mut sum_b = 0;
    lines.iter().for_each(|line| {
        sum_b += rev_extrap(line);
    });

    println!("{:?}", sum_a);
    println!("{:?}", sum_b);
}
