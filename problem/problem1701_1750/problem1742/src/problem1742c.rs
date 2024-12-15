use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _: String = read_string();
        let field: Vec<Vec<char>> = (0..8)
            .map(|_| read_string().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut res: char = ' ';
        for i in 0..8 {
            if (0..8).fold(true, |flag, j| flag && field[i][j] == 'R') {
                res = 'R';
            }
            if (0..8).fold(true, |flag, j| flag && field[j][i] == 'B') {
                res = 'B';
            }
        }

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
