use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, m): (usize, u16) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1] as u16)
    };

    let a: Vec<u16> = read_vec();

    let res: u16 = a
        .iter()
        .enumerate()
        .fold((0, 0), |(res, sum), (i, &x)| {
            if i == n - 1 {
                (res + if sum + x <= m { 1 } else { 2 }, 0)
            } else if sum + x <= m {
                (res, sum + x)
            } else {
                (res + 1, x)
            }
        })
        .0;

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
