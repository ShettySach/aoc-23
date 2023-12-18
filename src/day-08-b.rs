use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let data = include_str!("inputs.txt");
    let data: Vec<&str> = data.split("\n\n").collect();

    let directions: Vec<char> = data[0].chars().collect();

    let nodes: Vec<String> = data[1].lines().map(String::from).collect();

    let mut nmap: HashMap<String, (String, String)> = HashMap::new();
    let mut roots: Vec<String> = Vec::new();
    nodes.iter().for_each(|line| {
        let line: String = line.chars().filter(|&c| !"= (),".contains(c)).collect();
        let node = line[0..3].to_string();
        let left = line[3..6].to_string();
        let right = line[6..].to_string();
        nmap.insert(node.clone(), (left, right));

        if node.chars().skip(2).next().unwrap() == 'A' {
            roots.push(node);
        }
    });

    let mut directions = directions.iter().cycle();
    let mut stepvec: Vec<usize> = Vec::new();

    roots.iter().for_each(|root| {
        let mut steps = 0;
        let root = nmap.get_key_value(root).expect("No root node");
        let (mut node, mut val) = root;

        while !node.ends_with('Z') {
            let dir = directions.next().expect("No more directions");
            let (left, right) = val;
            let root = if *dir == 'L' {
                nmap.get_key_value(left).expect("Left node not found")
            } else {
                nmap.get_key_value(right).expect("Right node not found")
            };

            steps += 1;
            let (new_node, new_val) = root;

            node = new_node;
            val = new_val;
        }

        stepvec.push(steps);
    });

    let steps = stepvec.iter().fold(1, |a, &x| lcm(a, x));
    println!("{}", steps);
}
