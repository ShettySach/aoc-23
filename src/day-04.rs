use std::collections::HashSet;

fn first(lines: &Vec<String>) {
    let mut sum = 0;

    lines.iter().for_each(|line| {
        let mut line = line.split(" | ");

        let wins = line.next().unwrap();
        let mut wins: HashSet<&str> = wins.split(" ").collect();
        wins.remove("");

        let given = line.next().unwrap();
        let mut given: Vec<&str> = given.split(" ").collect();
        given.retain(|x| *x != "");

        let mut points: i32 = -1;

        given.iter().for_each(|num| {
            if wins.contains(num) {
                points += 1;
            }
        });

        if points > -1 {
            sum += i32::pow(2, points as u32);
        }
    });

    println!("{}", sum);
}

fn second(lines: Vec<String>) {
    let n = lines.len();

    let mut copies: Vec<usize> = vec![1; n];

    lines.iter().enumerate().for_each(|(i, line)| {
        let mut line = line.split(" | ");

        let wins = line.next().unwrap();
        let mut wins: HashSet<&str> = wins.split(" ").collect();
        wins.remove("");

        let given = line.next().unwrap();
        let mut given: Vec<&str> = given.split(" ").collect();
        given.retain(|x| *x != "");

        let mut points = 0;

        given.iter().for_each(|num| {
            if wins.contains(num) {
                points += 1;
            }
        });

        let end = usize::min(i + points + 1, n);
        for j in (i + 1)..end {
            copies[j] += copies[i];
        }
    });

    let sum: usize = copies.iter().sum();

    println!("{}", sum);
}

fn main() {
    let lines = std::fs::read_to_string("src/inputs.txt").unwrap();
    let lines: Vec<String> = lines.lines().map(|s| s[10..].to_string()).collect();

    first(&lines);
    second(lines);
}
