use std::io::{stdout, Write, BufWriter};

const N: usize = 8;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        read_string();

        let board: Vec<Vec<char>> = (0..N).map(|_| read_string().chars().collect()).collect();

        for i in 1..N-1 {
            for j in 1..N-1 {
                if board[i][j] != '#' {
                    continue;
                }

                if board[i-1][j+1] == '#'
                    && board[i+1][j+1] == '#'
                    && board[i+1][j-1] == '#'
                    && board[i-1][j-1] == '#' {
                        writeln!(out, "{} {}", i+1, j+1).unwrap();
                    }
            }
        }
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