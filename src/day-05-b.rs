use std::ops::Range;

#[derive(Debug)]
struct OffsetRange {
    offset: usize,
    pos: bool,
    range: Range<usize>,
}

fn parse_block(block: &str) -> Vec<OffsetRange> {
    let new_block: Vec<&str> = block.lines().skip(1).collect();
    let mut outvec: Vec<OffsetRange> = Vec::new();

    new_block.iter().for_each(|line| {
        let values: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Error in parsing"))
            .collect();

        let offset = values[0].abs_diff(values[1]);
        let pos = if values[0] >= values[1] { true } else { false };
        let range = values[1]..(values[1] + values[2]);

        let vals = OffsetRange { offset, pos, range };
        outvec.push(vals);
    });

    // println!("{:?}", outvec);
    outvec
}

fn mapper(seed: usize, seed_map: &Vec<OffsetRange>) -> usize {
    let mut result = None;

    seed_map.iter().for_each(|val| {
        if val.range.contains(&seed) {
            if val.pos {
                result = Some(seed + val.offset);
            } else {
                result = Some(seed - val.offset);
            }
        }
    });

    // print!("{:?}", result);
    result.unwrap_or(seed)
}

fn main() {
    let lines = std::fs::read_to_string("src/inputs.txt").unwrap();
    let lines: Vec<String> = lines.split("\n\n").map(|s| s.to_string()).collect();

    let seeds: Vec<usize> = lines[0][7..]
        .split(" ")
        .map(|s| s.parse().expect("Error in parsing"))
        .collect();

    let mut minim = usize::MAX;

    let map_1 = parse_block(&lines[1]);
    let map_2 = parse_block(&lines[2]);
    let map_3 = parse_block(&lines[3]);
    let map_4 = parse_block(&lines[4]);
    let map_5 = parse_block(&lines[5]);
    let map_6 = parse_block(&lines[6]);
    let map_7 = parse_block(&lines[7]);

    let duos = seeds.chunks(2);

    for duo in duos {
        if let &[start, end] = duo {
            (start..start + end).for_each(|s| {
                let s_1 = mapper(s, &map_1);
                let s_2 = mapper(s_1, &map_2);
                let s_3 = mapper(s_2, &map_3);
                let s_4 = mapper(s_3, &map_4);
                let s_5 = mapper(s_4, &map_5);
                let s_6 = mapper(s_5, &map_6);
                let s_7 = mapper(s_6, &map_7);

                minim = usize::min(minim, s_7);
            })
        }
    }

    println!("{}", minim);
}
