use linked_hash_map::LinkedHashMap;

fn hashfn(inp: &str) -> usize {
    let chars = inp.chars();
    let mut sum = 0;
    for ch in chars {
        sum += ch as usize;
        sum = (sum * 17) % 256;
    }
    sum
}

fn main() {
    let data = include_str!("inputs.txt");
    let data = data[..data.len() - 1].split(',').collect::<Vec<&str>>();
    let mut res = 0;

    data.iter().for_each(|dline| {
        res += hashfn(dline);
    });

    println!("Part 1 = {}", res);

    let mut boxes: Vec<LinkedHashMap<&str, usize>> = vec![LinkedHashMap::new(); 256];

    for dline in data {
        if dline.contains('=') {
            let mut dline = dline.split('=');
            let label = dline.next().unwrap();
            let lens = dline
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            let box_no = hashfn(label);

            if let Some(val) = boxes[box_no].get_mut(label) {
                *val = lens;
            } else {
                boxes[box_no].insert(label, lens);
            }
        } else {
            let label = &dline[..dline.len() - 1];
            let box_no = hashfn(label);
            boxes[box_no].remove(label);
        }
    }

    let mut res = 0;
    boxes.iter().enumerate().for_each(|(i, boxe)| {
        boxe.values().enumerate().for_each(|(j, lens)| {
            res += (j + 1) * (i + 1) * lens;
        })
    });

    println!("Part 2 = {}", res);
}
