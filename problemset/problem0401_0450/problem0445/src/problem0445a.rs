use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<usize> = read_vec();
    let h: usize = input[0];
    let w: usize = input[1];

    let mut out = BufWriter::new(stdout().lock());

    for i in 0..h {
        let mut line: Vec<char> = read_string().chars().collect();

        for j in 0..w {
            if line[j] == '-' {
                continue;
            }

            if (i + j) % 2 == 0 {
                line[j] = 'B';
            }
            else {
                line[j] = 'W';
            }
        }

        let res: String =
            line
            .iter()
            .fold(String::new(), |res, x| res + &x.to_string());

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