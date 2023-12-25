fn main() {
    let lines = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<String> = lines.lines().map(|s| s.to_string()).collect();

    let mut sum = 0;

    lines.iter().for_each(|line| {
        let mut line = line.split(":");

        let (mut r, mut g, mut b) = (1, 1, 1);

        let rounds_str = line.skip(1).next().unwrap();
        let rounds_str = rounds_str.split(";").collect::<Vec<&str>>();

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
                            r = cube_count;
                        }
                    }
                    "green" => {
                        if cube_count > g {
                            g = cube_count;
                        }
                    }
                    "blue" => {
                        if cube_count > b {
                            b = cube_count;
                        }
                    }
                    _ => {}
                }
            })
        });

        sum += r * g * b;
    });

    println!("{}", sum);
}
