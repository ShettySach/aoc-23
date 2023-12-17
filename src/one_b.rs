use std::fs;

fn main() {
    let file_path = "src/input.txt";
    println!("In file {}", file_path);

    let lines = fs::read_to_string(file_path).expect("Failed to read file");

    let lines = lines
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .replace("zero", "z0o");

    let mut sum = 0;

    for lin in lines.lines() {
        let mut num = 0;
        for first in lin.chars() {
            if let Some(digit) = first.to_digit(10) {
                num += 10 * digit;
                break;
            }
        }

        for first in lin.chars().rev() {
            if let Some(digit) = first.to_digit(10) {
                num += digit;
                break;
            }
        }

        sum += num;

        println!("{}", num);
    }

    println!("\n{}", sum);
}
