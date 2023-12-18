use std::fs;
use std::ops::Range;

fn get_numbers(line: String) -> Vec<(usize, Range<usize>, bool)> {
    let chars: Vec<_> = line.chars().collect();
    let mut i = 0;
    let mut result = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            let start = i;
            let mut val = 0;
            while i < chars.len() && chars[i].is_ascii_digit() {
                val *= 10;
                val += chars[i] as usize - '0' as usize;
                i += 1;
            }
            result.push((val, start..i, false));
        } else {
            i += 1;
        }
    }
    result
}

fn main() {
    let lines: Vec<_> = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut numbers = Vec::new();

    for line in lines.iter() {
        let line = line.trim();
        numbers.push(get_numbers(line.to_string()));
    }

    for (r, line) in lines.iter().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            if c != '.' && !c.is_ascii_digit() {
                let mut to_check = vec![r];
                if r > 0 {
                    to_check.push(r - 1);
                }
                if r < numbers.len() {
                    to_check.push(r + 1);
                }
                for row in to_check {
                    for (_, range, included) in numbers[row].iter_mut() {
                        if range.contains(&col)
                            || range.contains(&(col - 1))
                            || range.contains(&(col + 1))
                        {
                            *included = true;
                        }
                    }
                }
            }
        }
    }

    let part1: usize = numbers
        .iter()
        .map(|row| {
            row.iter()
                .filter(|(_, _, included)| *included)
                .map(|(num, _, _)| num)
                .sum::<usize>()
        })
        .sum();
    dbg!(part1);

    let mut part2 = 0;
    for (r, line) in lines.into_iter().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            if c == '*' {
                let mut nums = Vec::new();
                let mut to_check = vec![r];
                if r > 0 {
                    to_check.push(r - 1);
                }
                if r < numbers.len() {
                    to_check.push(r + 1);
                }
                for row in to_check {
                    for (num, range, _) in numbers[row].iter() {
                        if range.contains(&col)
                            || range.contains(&(col - 1))
                            || range.contains(&(col + 1))
                        {
                            nums.push(*num);
                        }
                    }
                }
                if nums.len() == 2 {
                    part2 += nums[0] * nums[1];
                }
            }
        }
    }
    dbg!(part2);
}
