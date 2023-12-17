use std::collections::HashMap;

fn cardsort<'a>(
    (a_str, _): &'a (&str, usize),
    (b_str, _): &'a (&str, usize),
) -> std::cmp::Ordering {
    let strengths = HashMap::from([('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);

    let result = std::iter::zip(a_str.chars(), b_str.chars()).find_map(|(a, b)| {
        let a_val = match a.to_digit(10) {
            Some(val) => val,
            None => *strengths.get(&a).unwrap(),
        };
        let b_val = match b.to_digit(10) {
            Some(val) => val,
            None => *strengths.get(&b).unwrap(),
        };

        if a_val > b_val {
            Some(std::cmp::Ordering::Greater)
        } else if b_val > a_val {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    });

    result.unwrap_or(std::cmp::Ordering::Equal)
}

fn main() {
    let lines = std::fs::read_to_string("src/inputs.txt").unwrap();
    let lines: Vec<String> = lines.lines().map(|s| s.to_string()).collect();

    let mut types: Vec<Vec<(&str, usize)>> = vec![
        Vec::new(), // fives
        Vec::new(), // fours
        Vec::new(), // fulls
        Vec::new(), // threes
        Vec::new(), // twos
        Vec::new(), // ones
        Vec::new(), // highs
    ];

    lines.iter().for_each(|line| {
        let mut line = line.split(" ");

        let hand = line.next().unwrap();
        let bid: usize = line.next().unwrap().parse().unwrap();

        let mut occurs: HashMap<char, usize> = HashMap::new();

        hand.chars().into_iter().for_each(|ch| {
            occurs
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        match occurs.len() {
            1 => {
                types[6].push((hand, bid));
            }
            2 => {
                let prod: usize = occurs.values().product();
                if prod == 4 {
                    types[5].push((hand, bid));
                } else {
                    types[4].push((hand, bid));
                }
            }
            3 => {
                let prod: usize = occurs.values().product();
                if prod == 3 {
                    types[3].push((hand, bid));
                } else {
                    types[2].push((hand, bid));
                }
            }
            4 => {
                types[1].push((hand, bid));
            }
            _ => {
                types[0].push((hand, bid));
            }
        }
    });

    types.iter_mut().for_each(|typ| {
        typ.sort_by(|a, b| cardsort(a, b));
    });

    let res: usize = types
        .concat()
        .iter()
        .enumerate()
        .map(|(index, &(_, value))| (index + 1) * value)
        .sum();

    println!("{}", res);
}
