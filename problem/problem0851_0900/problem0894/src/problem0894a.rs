use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: String = read_string();

    let l: usize = n.len();

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 3]; l + 1];
    for (i, c) in n.chars().enumerate() {
        dp[i + 1] = dp[i].clone();

        if c == 'Q' {
            dp[i + 1][0] += 1;
            dp[i + 1][2] += dp[i][1];
        } else if c == 'A' {
            dp[i + 1][1] += dp[i][0];
        }
    }

    let res: i64 = dp[l][2];

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", res).unwrap();
}

#[allow(dead_code)]
fn read_string() -> String {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().to_string()
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    read_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read_string()
        .split_whitespace()
        .map(|v| v.parse().ok().unwrap())
        .collect()
}
