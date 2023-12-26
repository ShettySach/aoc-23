use hashbrown::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn ngbrs(&self) -> Vec<Self> {
        let moves: Vec<(isize, isize)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        moves
            .iter()
            .map(|(mx, my)| {
                let mut temp = self.clone();
                temp.x = ((self.x as isize) + mx) as usize;
                temp.y = ((self.y as isize) + my) as usize;
                temp
            })
            .collect()
    }
}

fn main() {
    let data: Vec<Vec<char>> = include_str!("inputs.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start = Point { x: 0, y: 0 };
    let mut grid: HashSet<Point> = HashSet::new();

    for (i, row) in data.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '.' {
                grid.insert(Point { x: i, y: j });
            } else if cell == 'S' {
                start = Point { x: i, y: j };
            }
        }
    }

    let res = bfs(&grid, &start, 64);
    println!("Part 1 = {}", res);
}

// Used help from this soln to optimise - https://github.com/wilkotom/Aoc2023/blob/main/day21/src/main.rs

fn bfs(grid: &HashSet<Point>, start: &Point, max_step: usize) -> usize {
    let mut vis = HashSet::new();
    let mut next: VecDeque<(Point, usize)> = VecDeque::from(vec![(*start, 0)]);
    let mut res = 0;

    let bounds: Point = Point {
        x: grid.iter().map(|c| c.x).max().unwrap(),
        y: grid.iter().map(|c| c.y).max().unwrap(),
    };

    while let Some((locn, steps_taken)) = next.pop_front() {
        if vis.contains(&(locn)) {
            continue;
        }
        vis.insert(locn);

        if steps_taken % 2 == max_step % 2 {
            res += 1;
        }

        if steps_taken == max_step {
            continue;
        }

        for ngbr in locn.ngbrs() {
            if grid.contains(&Point {
                x: ngbr.x.rem_euclid(bounds.x + 1),
                y: ngbr.y.rem_euclid(bounds.y + 1),
            }) {
                next.push_back((ngbr, steps_taken + 1));
            }
        }
    }

    res
}
