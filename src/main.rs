use std::collections::HashMap;

fn main() {
    let data = include_str!("tests.txt");
    let data: Vec<&str> = data.split("\n\n").collect();

    let directions: Vec<char> = data[0].chars().collect();

    let nodes: Vec<String> = data[1].lines().map(String::from).collect();

    let mut nmap: HashMap<String, (String, String)> = HashMap::new();
    nodes.iter().for_each(|line| {
        let line: String = line.chars().filter(|&c| !"= (),".contains(c)).collect();
        let node = line[0..3].to_string();
        let left = line[3..6].to_string();
        let right = line[6..].to_string();
        nmap.insert(node, (left, right));
    });

    let root = nmap.get_key_value("AAA").expect("No root node"); // Use clone() here
    let (mut node, mut val) = root;

    let mut directions = directions.iter().cycle();

    while node != "ZZZ" {
        let dir = directions.next().expect("No more directions");
        let (left, right) = val;
        let root = if *dir == 'L' {
            nmap.get_key_value(left)
                .expect("Left node not found")
                .clone() // Use clone() here
        } else {
            nmap.get_key_value(right)
                .expect("Right node not found")
                .clone() // Use clone() here
        };

        let (new_node, new_val) = root;

        node = new_node;
        val = new_val;
    }

    print!("{}", node);
}
