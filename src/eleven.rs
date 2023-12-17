// https://github.com/NikolaIliev/aoc_rust/blob/master/src/bin/year2023_day11.rs

fn find_dist(input: &str, exp: usize) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let vert_dists: Vec<_> = (0..grid.len())
        .map(|r| grid[r].iter().all(|&c| c == '.') as usize * (exp - 1) + 1)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let horz_dists: Vec<_> = (0..grid[0].len())
        .map(|c| grid.iter().all(|row| row[c] == '.') as usize * (exp - 1) + 1)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let galaxies = find_galaxies(grid);
    let mut dists = 0;
    for (i, &(r0, c0)) in galaxies.iter().enumerate() {
        for &(r1, c1) in galaxies.iter().skip(i + 1) {
            dists += vert_dists[r0.max(r1)] - vert_dists[r0.min(r1)];
            dists += horz_dists[c0.max(c1)] - horz_dists[c0.min(c1)];
        }
    }
    dists
}
fn find_galaxies(grid: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, ch)| {
            if *ch == '#' {
                res.push((i, j));
            }
        })
    });
    res
}

fn main() {
    let data = include_str!("inputs.txt");

    let galaxies_p1 = find_dist(data, 2);
    println!("{:?}", galaxies_p1);

    let galaxies_p2 = find_dist(data, 1_000_000);
    println!("{:?}", galaxies_p2);
}
