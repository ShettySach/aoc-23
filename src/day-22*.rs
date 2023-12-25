use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn get(&self) -> (usize, usize, usize) {
        (self.x, self.y, self.z)
    }

    fn put(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Brick {
    start: Coordinate,
    end: Coordinate,
}

fn main() {
    let mut bricks: Vec<Brick> = include_str!("inputs.txt")
        .lines()
        .map(|l| {
            let mut l = l.split("~");
            let mut start = l.next().unwrap().split(",");
            let start = Coordinate {
                x: start.next().unwrap().parse().unwrap(),
                y: start.next().unwrap().parse().unwrap(),
                z: start.next().unwrap().parse().unwrap(),
            };
            let mut end = l.next().unwrap().split(",");
            let end = Coordinate {
                x: end.next().unwrap().parse().unwrap(),
                y: end.next().unwrap().parse().unwrap(),
                z: end.next().unwrap().parse().unwrap(),
            };
            Brick { start, end }
        })
        .collect();

    bricks.sort_unstable_by_key(|brick| brick.start.z);

    let mut adjacent = vec![(HashSet::new(), HashSet::new()); bricks.len()];
    let mut space = HashMap::new();

    for i in 0..bricks.len() {
        let (x1, y1, mut z1) = bricks[i].start.get();
        let (x2, y2, mut z2) = bricks[i].end.get();

        while z1 > 1
            && (x1..=x2)
                .cartesian_product(y1..=y2)
                .all(|(x, y)| !space.contains_key(&(x, y, z1 - 1)))
        {
            z2 -= 1;
            z1 -= 1;
        }

        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            for z in z1..=z2 {
                space.insert((x, y, z), i);
            }
            if let Some(&j) = space.get(&(x, y, z1 - 1)) {
                adjacent[j].0.insert(i);
                adjacent[i].1.insert(j);
            }
        }

        bricks[i].start = Coordinate::put(x1, y1, z1);
        bricks[i].end = Coordinate::put(x2, y2, z2);
    }

    let mut falling = HashSet::new();
    let (mut p1, mut p2) = (0, 0);

    for b in 0..bricks.len() {
        falling.clear();
        disintegrate_all(&adjacent, &mut falling, b);
        p1 += (falling.len() == 1) as usize;
        p2 += falling.len() - 1;
    }
    println!("Part 1 = {} \nPart 2 = {}", p1, p2);
}

// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/22.rs

fn disintegrate_all(
    adjacent: &[(HashSet<usize>, HashSet<usize>)],
    falling: &mut HashSet<usize>,
    i: usize,
) {
    falling.insert(i);
    for &above in &adjacent[i].0 {
        if adjacent[above].1.iter().all(|x| falling.contains(x)) {
            disintegrate_all(adjacent, falling, above);
        }
    }
}
