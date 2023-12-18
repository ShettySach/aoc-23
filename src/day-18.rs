#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct DirSteps {
    dir: Direction,
    steps: usize,
}

impl Direction {
    fn to_val(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

fn shoelace(data: Vec<DirSteps>) -> isize {
    let (mut px, mut _py) = (0, 0);
    let mut perimeter = 0;
    let mut area = 0;

    for ds in data.iter() {
        let (mut cx, mut cy) = ds.dir.to_val();
        cx *= ds.steps as isize;
        cy *= ds.steps as isize;
        px += cx;
        _py += cy;

        perimeter += ds.steps as isize;
        area += px * cy;
    }

    area + perimeter / 2 + 1
}

fn main() {
    let part_a: Vec<DirSteps> = include_str!("inputs.txt")
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            let dir = match l.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("?dir"),
            };
            let steps = l.next().unwrap().parse().unwrap();
            DirSteps { dir, steps }
        })
        .collect();

    println!("Part 1 = {}", shoelace(part_a));

    let part_b: Vec<DirSteps> = include_str!("inputs.txt")
        .lines()
        .map(|l| {
            let l: String = l
                .split_whitespace()
                .last()
                .unwrap()
                .chars()
                .filter(|&c| !"#()".contains(c))
                .collect();
            let dir = match l.chars().last().unwrap() {
                '3' => Direction::Up,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '0' => Direction::Right,
                _ => panic!("?dir"),
            };
            let steps = &l[0..l.len() - 1];
            let steps = usize::from_str_radix(&steps, 16).unwrap();
            DirSteps { dir, steps }
        })
        .collect();

    println!("Part 2 = {}", shoelace(part_b));
}
