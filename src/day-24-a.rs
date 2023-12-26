// Thanks to - https://www.youtube.com/watch?v=guOyA7Ijqgk && https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day24/part1.rs

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Position,
    vel: Velocity,
}

impl Hailstone {
    fn coeffs(&self) -> (f64, f64, f64) {
        let a = self.vel.y;
        let b = -self.vel.x;
        let c = self.vel.y * self.pos.x - self.vel.x * self.pos.y;

        (a, b, c)
    }

    fn parallel(&self, other: &Hailstone) -> bool {
        self.vel.y * other.vel.x == self.vel.x * other.vel.y
    }

    fn intersection(&self, other: &Hailstone) -> Option<(f64, f64)> {
        if self.parallel(other) {
            return None;
        }

        let (sa, sb, sc) = self.coeffs();
        let (oa, ob, oc) = other.coeffs();

        let x = (ob * sc - sb * oc) / (sa * ob - sb * oa);
        let y = (sa * oc - oa * sc) / (sa * ob - oa * sb);

        let intersects_in_future = (x - self.pos.x < 0.0) == (self.vel.x < 0.0)
            && (y - self.pos.y < 0.0) == (self.vel.y < 0.0)
            && (x - other.pos.x < 0.0) == (other.vel.x < 0.0)
            && (y - other.pos.y < 0.0) == (other.vel.y < 0.0);

        match intersects_in_future {
            true => Some((x, y)),
            false => None,
        }
    }
}

const L_LIMIT: f64 = 200000000000000.0;
const U_LIMIT: f64 = 400000000000000.0;
// const L_LIMIT: f64 = 7.0;
// const U_LIMIT: f64 = 27.0;

fn part_a(hailstones: Vec<Hailstone>) -> usize {
    let mut res = 0;

    for (ind, hailstone1) in hailstones.iter().enumerate() {
        for hailstone2 in hailstones.iter().skip(ind + 1) {
            if let Some((ix, iy)) = hailstone1.intersection(hailstone2) {
                if ix >= L_LIMIT && ix <= U_LIMIT && iy >= L_LIMIT && iy <= U_LIMIT {
                    res += 1;
                }
            }
        }
    }

    res
}

fn main() {
    let hailstones: Vec<Hailstone> = include_str!("inputs.txt")
        .lines()
        .map(|l| {
            let mut l = l.split('@');

            let mut pos = l.next().unwrap().split(", ");
            let pos = Position {
                x: pos.next().unwrap().trim().parse().unwrap(),
                y: pos.next().unwrap().trim().parse().unwrap(),
                z: pos.next().unwrap().trim().parse().unwrap(),
            };

            let mut vel = l.next().unwrap().trim().split(", ");
            let vel = Velocity {
                x: vel.next().unwrap().trim().parse().unwrap(),
                y: vel.next().unwrap().trim().parse().unwrap(),
                z: vel.next().unwrap().trim().parse().unwrap(),
            };

            Hailstone { pos, vel }
        })
        .collect();

    let res = part_a(hailstones);
    println!("Part A = {}", res);
}
