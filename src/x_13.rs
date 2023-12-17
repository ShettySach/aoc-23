fn reflection(lines: Vec<&str>) -> (bool, usize) {
    let mut res = 0;
    let mut mirror: Vec<&str> = vec![lines[0]];
    let mut flag = true;

    lines.iter().skip(1).enumerate().for_each(|(i, line)| {
        if let Some(val) = mirror.last() {
            if line == val {
                mirror.pop();
                if flag {
                    res = i + 1;
                    flag = false;
                }
            } else {
                mirror.push(&line);
            }
        } else {
            return;
        }
    });

    let (mir_emp, res) = if mirror.len() == 0 {
        (true, res)
    } else {
        (false, 0)
    };
    (mir_emp, res)
}

fn horiz_mirror(patt: &str) -> usize {
    let hlines: Vec<&str> = patt.lines().collect();

    let (forw, fres) = reflection(hlines.clone());
    let hrev: Vec<_> = hlines.iter().rev().cloned().collect();
    let (backw, bres) = reflection(hrev);

    let res = if forw {
        fres
    } else if backw {
        hlines.len() - bres
    } else {
        0
    };

    res
}

fn vert_mirror(patt: &str) -> usize {
    let plines: Vec<_> = patt.lines().collect();
    let mut vlines: Vec<String> = vec![String::new(); plines[0].len()];
    plines.iter().for_each(|pline| {
        for (i, c) in pline.chars().enumerate() {
            vlines[i].push(c);
        }
    });

    let vlines: Vec<&str> = vlines.iter().map(|s| &s[..]).collect();

    let (forw, fres) = reflection(vlines.clone());
    let vrev: Vec<_> = vlines.iter().rev().cloned().collect();
    let (backw, bres) = reflection(vrev);

    let res = if forw {
        fres
    } else if backw {
        vlines.len() - bres
    } else {
        0
    };

    res
}

fn main() {
    let data: Vec<_> = include_str!("tests.txt").split("\n\n").collect();
    let mut res: usize = 0;

    data.iter().for_each(|patt| {
        let horiz = horiz_mirror(patt);
        let vert = vert_mirror(patt);

        // let addval = if horiz > vert { 100 * horiz } else { vert };
        // res += addval;
        println!("{} {}", horiz, vert);
        res += horiz * 100 + vert;
    });

    println!("{}", res);
}
