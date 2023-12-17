//Thanks to https://github.com/akaritakai/AdventOfCode2023/blob/main/src/day12.rs

fn count_arrangements(line: &str, counts: Vec<usize>) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }

    dp[0][0][0]
}

fn main() {
    let data: Vec<String> = include_str!("inputs.txt")
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    data.iter().for_each(|line| {
        let line: Vec<&str> = line.split(' ').collect();

        let line1 = line[0];
        let vals1: Vec<usize> = line[1].split(',').map(|v| v.parse().unwrap()).collect();
        sum_p1 += count_arrangements(line1, vals1.clone());

        let line2 = format!("{line1}?{line1}?{line1}?{line1}?{line1}");
        let vals2: Vec<usize> = vals1
            .iter()
            .cycle()
            .take(vals1.len() * 5)
            .cloned()
            .collect();
        sum_p2 += count_arrangements(&line2, vals2.clone());
    });

    println!("{}", sum_p1);
    println!("{}", sum_p2);
}
