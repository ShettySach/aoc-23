use std::collections::{HashMap, VecDeque};
const ITERS: usize = 200;

fn north(mut data: Vec<String>) -> Vec<String> {
    let mut emptys: Vec<VecDeque<usize>> = vec![VecDeque::new(); data[0].len()];
    let n = data.len();

    for i in 0..n {
        // let chars: Vec<_> = data[i].chars().collect();
        let mut chars: Vec<_> = data[i].chars().collect();

        for j in 0..chars.len() {
            if chars[j] == 'O' {
                if let Some(val) = emptys[j].pop_front() {
                    chars[j] = '.';
                    let mut temp_chars: Vec<_> = data[val].chars().collect();
                    temp_chars[j] = 'O';
                    data[val] = temp_chars.iter().collect();
                    emptys[j].push_back(i);
                }
            } else if chars[j] == '#' {
                emptys[j] = VecDeque::new();
            } else {
                emptys[j].push_back(i);
            }
        }
        data[i] = chars.iter().collect();
    }

    data
}

fn west_east(mut data: Vec<String>, east: bool) -> Vec<String> {
    data.iter_mut().for_each(|lin| {
        let mut emptys: VecDeque<usize> = VecDeque::new();
        let mut chars: Vec<_> = lin.chars().collect();

        let char_range: Vec<_> = if east {
            (0..chars.len()).rev().collect()
        } else {
            (0..chars.len()).collect()
        };

        for j in char_range {
            if chars[j] == 'O' {
                if let Some(val) = emptys.pop_front() {
                    chars[j] = '.';
                    chars[val] = 'O';
                    emptys.push_back(j);
                }
            } else if chars[j] == '#' {
                emptys = VecDeque::new();
            } else {
                emptys.push_back(j);
            }
        }

        *lin = chars.iter().collect();
    });

    data
}

fn tilt_cycle(d: Vec<String>, iters: usize, mut saved: HashMap<usize, usize>) -> Vec<String> {
    let n = north(d);
    let mut w = west_east(n, false);
    w.reverse();
    let mut s = north(w);
    s.reverse();
    let e = west_east(s, true);

    let l = load(&e);

    if let Some(prev) = saved.remove(&l) {
        println!("{} {}", ITERS - iters, l);
        saved.insert(l, iters);
        println!(
            "Current = {}, Prev = {}, Val = {}",
            ITERS - iters,
            ITERS - prev,
            l
        );
        println!("Diff = {}", prev - iters);
    } else {
        println!("{} {}", ITERS - iters, l);
        saved.insert(l, iters);
    }

    let retval = if iters == 0 {
        e
    } else {
        // println!("{}", saved.len());
        tilt_cycle(e, iters - 1, saved)
    };

    retval
}

fn load(data: &Vec<String>) -> usize {
    let mut res = 0;
    let n = data.len();

    data.iter().enumerate().for_each(|(i, lin)| {
        lin.chars().for_each(|ch| {
            if ch == 'O' {
                res += n - i;
            }
        })
    });

    res
}

fn main() {
    // let data: Vec<String> = include_str!("inputs.txt")
    let data: Vec<String> = include_str!("inputs.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();

    // let iters = repet(1_000_000_000);
    // println!("{}", iters);

    let saved: HashMap<usize, usize> = HashMap::new();
    let data = tilt_cycle(data, ITERS, saved);

    //After 110, cycle repeats in 18 steps.
    let true_iters = (1_000_000_000 - 111) % 18;
    //(1_000_000_000 - 111) % 18 = 7
    let true_iters = 111 + true_iters - 1;
    //7 steps after 111 = 117
    let saved: HashMap<usize, usize> = HashMap::new();
    let true_data = tilt_cycle(data, true_iters, saved);
    //Therefore, load after 117 iterations = load after 1 billion iterations

    println!("{:?}", load(true_data));
}
