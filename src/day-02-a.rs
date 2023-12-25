fn main() {
    let lines = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<String> = lines.lines().map(|s| s.to_string()).collect();

    let (r, g, b) = (12, 13, 14);

    let mut sum = 0;
    lines.iter().enumerate().for_each(|i, line| {
        let mut line = line.split(":");

        let game_id = i + 1;

        let rounds_str = line.next().unwrap();
        let rounds_str = rounds_str.split(";").collect::<Vec<&str>>();

        let mut valid = true;
        rounds_str.iter().for_each(|round_str| {
            let cubes_str = round_str.split(",").collect::<Vec<&str>>();
            cubes_str.iter().for_each(|cube_str| {
                let cube_str = cube_str.trim().split(" ").collect::<Vec<&str>>();
                let cube_color = cube_str[1];
                let cube_count_str = cube_str[0];
                let cube_count = cube_count_str.parse::<u32>().unwrap();

                match cube_color {
                    "red" => {
                        if cube_count > r {
                            valid = false;
                        }
                    }
                    "green" => {
                        if cube_count > g {
                            valid = false;
                        }
                    }
                    "blue" => {
                        if cube_count > b {
                            valid = false;
                        }
                    }
                    _ => {}
                }
            })
        });

        if valid {
            sum += game_id;
        }
    });

    println!("{}", sum);
}
