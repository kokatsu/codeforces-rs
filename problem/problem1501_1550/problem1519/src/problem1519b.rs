use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let m: usize = input[1];
        let k: usize = input[2];

        let mut dp: Vec<Vec<usize>> = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if i + 1 < n {
                    dp[i+1][j] = dp[i][j] + j + 1;
                }
                if j + 1 < m {
                    dp[i][j+1] = dp[i][j] + i + 1;
                }
            }
        }

        let res: &str = if dp[n-1][m-1] == k {"YES"} else {"NO"};

        writeln!(out, "{}", res).unwrap();
    }
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