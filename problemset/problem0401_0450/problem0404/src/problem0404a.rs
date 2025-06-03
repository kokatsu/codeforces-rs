use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let paper: Vec<Vec<char>> = (0..n)
        .map(|_| read_string().chars().collect::<Vec<char>>())
        .collect();

    let mut ok: bool = paper[0][0] != paper[0][1];
    for i in 0..n {
        for j in 0..n {
            if i == j || i + j == n - 1 {
                if paper[i][j] != paper[0][0] {
                    ok = false;
                    break;
                }
            } else if paper[i][j] != paper[0][1] {
                ok = false;
                break;
            }
        }
    }

    let res: &str = if ok { "YES" } else { "NO" };

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
