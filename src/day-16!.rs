// Attempt at solution. Does not escape cycle case.

use std::collections::{HashSet, VecDeque};

fn dir_val(dir: char) -> (isize, isize) {
    let res = match dir {
        'r' => (0, 1),
        'l' => (0, -1),
        'u' => (-1, 0),
        'd' => (1, 0),
        _ => panic!("?"),
    };

    res
}

fn mirror(mir: char, dir: char) -> char {
    let res = if mir == '\\' {
        match dir {
            'r' => 'd',
            'l' => 'u',
            'u' => 'l',
            'd' => 'r',
            _ => panic!("?"),
        }
    } else if mir == '/' {
        match dir {
            'r' => 'u',
            'l' => 'd',
            'u' => 'r',
            'd' => 'l',
            _ => panic!("?"),
        }
    } else {
        panic!("?")
    };

    res
}

fn main() {
    let data = include_str!("inputs.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut starts: VecDeque<((usize, usize), char)> = vec![((0, 0), 'r')].into();
    let mut vis_starts: HashSet<((usize, usize), char)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(((mut i, mut j), mut dir)) = starts.pop_front() {
        while let Some(ch) = data.get(i).and_then(|row| row.get(j)) {
            if *ch == '.' {
                visited.insert((i, j));
                println!("Added {} {} {}", i, j, ch);
            } else if *ch == '/' || *ch == '\\' {
                visited.insert((i, j));
                println!("Added {} {} {}", i, j, ch);
                println!("Dir change from {}", dir);
                dir = mirror(*ch, dir);
                println!("to {}", dir);
            } else {
                if *ch == '|' {
                    if dir == 'r' || dir == 'l' {
                        if !vis_starts.contains(&((i, j), 'u')) {
                            starts.push_back(((i, j), 'u'));
                            vis_starts.insert(((i, j), 'u'));
                        }
                        if !vis_starts.contains(&((i, j), 'd')) {
                            starts.push_back(((i, j), 'd'));
                            vis_starts.insert(((i, j), 'd'));
                        }
                        break;
                    } else {
                        visited.insert((i, j));
                        println!("Added {} {} {}", i, j, ch);
                    }
                } else if *ch == '-' {
                    if dir == 'u' || dir == 'd' {
                        if !vis_starts.contains(&((i, j), 'l')) {
                            starts.push_back(((i, j), 'l'));
                            vis_starts.insert(((i, j), 'l'));
                        }
                        if !vis_starts.contains(&((i, j), 'r')) {
                            starts.push_back(((i, j), 'r'));
                            vis_starts.insert(((i, j), 'r'));
                        }
                        break;
                    } else {
                        visited.insert((i, j));
                        println!("Added {} {} {}", i, j, ch);
                    }
                }
            }
            // println!("{:?}", starts);
            let (di, dj) = dir_val(dir);
            i = (i as isize + di) as usize;
            j = (j as isize + dj) as usize;
        }
    }

    println!("{}", visited.len());
}
