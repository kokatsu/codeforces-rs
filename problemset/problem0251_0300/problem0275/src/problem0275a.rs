use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = 3;
    let dx: Vec<usize> = vec![0, !0, 0, 1, 0];
    let dy: Vec<usize> = vec![0, 0, 1, 0, !0];

    let mut grid: Vec<Vec<usize>> = vec![vec![1; 3]; 3];
    for i in 0..n {
        let row: Vec<usize> = read_vec();
        for j in 0..n {
            let a: usize = row[j] % 2;
            if a == 0 {
                continue;
            }

            for k in 0..5 {
                let x: usize = i.wrapping_add(dx[k]);
                let y: usize = j.wrapping_add(dy[k]);
                if x < n && y < n {
                    grid[x][y] = 1 - grid[x][y];
                }
            }
        }
    }

    for i in 0..n {
        let res: String =
            grid[i]
            .iter()
            .fold(String::new(), |res, g| res + &g.to_string());

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