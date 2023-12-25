//Soln - https://github.com/apprenticewiz/adventofcode/blob/main/2023/rust/day21b/src/main.rs

use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32);

#[derive(Clone, Debug)]
struct Garden {
    rocks: HashSet<Position>,
    start: Position,
    extents: Position,
}

fn usage() {
    let progname = env::args().next().unwrap();
    println!("usage: {progname} <file>");
    process::exit(1);
}

fn build_garden(contents: &str) -> Garden {
    let mut rocks: HashSet<Position> = HashSet::new();
    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    rocks.insert(Position(row as i32, col as i32));
                }
                _ => {}
            };
        }
    }
    let num_rows = contents.lines().count() as i32;
    let num_cols = contents.lines().next().unwrap().chars().count() as i32;
    let mut expanded_rocks: HashSet<Position> = HashSet::new();
    for rock in rocks.iter() {
        for row_mul in 0..5 {
            for col_mul in 0..5 {
                let expanded_rock = Position(
                    rock.0 + (num_rows * row_mul as i32),
                    rock.1 + (num_cols * col_mul as i32),
                );
                expanded_rocks.insert(expanded_rock);
            }
        }
    }
    Garden {
        rocks: expanded_rocks,
        start: Position(num_rows * 5 / 2, num_cols * 5 / 2),
        extents: Position(num_rows * 5, num_cols * 5),
    }
}

fn in_bounds(pos: Position, extents: Position) -> bool {
    pos.0 >= 0 && pos.0 <= extents.0 && pos.1 >= 0 && pos.1 < extents.1
}

fn walk(garden: &Garden, steps: u32) -> u32 {
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(garden.start);
    for _i in 0..steps {
        let mut new_visited = HashSet::new();
        for pos in visited.iter().clone() {
            let neighbors = [
                Position(pos.0 - 1, pos.1),
                Position(pos.0 + 1, pos.1),
                Position(pos.0, pos.1 - 1),
                Position(pos.0, pos.1 + 1),
            ]
            .into_iter()
            .filter(|p| in_bounds(*p, garden.extents) && !garden.rocks.contains(p));
            for neighbor in neighbors {
                new_visited.insert(neighbor);
            }
        }
        visited = new_visited;
    }
    visited.len() as u32
}

fn process(contents: &str) -> i64 {
    let garden = build_garden(contents);
    let b0: i64 = walk(&garden, 65) as i64;
    let b1: i64 = walk(&garden, 65 + 131) as i64;
    let b2: i64 = walk(&garden, 65 + 2 * 131) as i64;
    let n: i64 = 202300;
    // the following formula comes from inv(A) * B = X,
    // where A is Vandemonde matrix:
    // [ 0 0 1 ]
    // [ 1 1 1 ]
    // [ 4 2 1 ]
    // and B is a column vector from the above values b0, b1, b2
    // credit to: https://gist.github.com/dllu/0ca7bfbd10a199f69bcec92f067ec94c
    // below uses Cramer's Rule to solve for x0, x1, x2
    let det_a: f64 = -2.0;
    let det_a0: f64 = -b0 as f64 + 2.0 * b1 as f64 - b2 as f64;
    let det_a1: f64 = 3.0 * b0 as f64 - 4.0 * b1 as f64 + b2 as f64;
    let det_a2: f64 = -2.0 * b0 as f64;
    let x0: i64 = (det_a0 / det_a) as i64;
    let x1: i64 = (det_a1 / det_a) as i64;
    let x2: i64 = (det_a2 / det_a) as i64;
    x0 * n * n + x1 * n + x2
}

fn main() {
    if env::args().count() < 2 {
        usage();
    }
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).expect("read of input file failed");
    let result = process(&contents);
    println!("result = {result}");
}
