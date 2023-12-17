use std::collections::{HashSet, VecDeque};

fn dir_val(dir: char) -> (isize, isize) {
    let res = match dir {
        'r' => (0, 1),
        'l' => (0, -1),
        'u' => (1, 0),
        'd' => (-1, 0),
        _ => (0, 0),
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
            _ => '?',
        }
    } else {
        match dir {
            'r' => 'u',
            'l' => 'd',
            'u' => 'r',
            'd' => 'l',
            _ => '?',
        }
    };

    res
}

fn main() {
    let data = include_str!("tests.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", data);

    let mut starts: VecDeque<((usize, usize), char)> = vec![((0, 0), 'r')].into();
    let mut visited: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);

    while let Some(((mut i, mut j), mut dir)) = starts.pop_front() {
        let (di, dj) = dir_val(dir);
        i = (i as isize + di) as usize;
        j = (j as isize + dj) as usize;

        if let Some(row) = data.get(i) {
            if let Some(ch) = row.get(j) {
                if *ch == '.' {
                    visited.insert((i, j));
                };
            }
        }
    }
}
