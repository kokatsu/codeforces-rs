use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let mut x: Vec<i16> = vec![0; n];
    let mut y: Vec<i16> = vec![0; n];
    for i in 0..n {
        let input: Vec<i16> = read_vec();
        x[i] = input[0];
        y[i] = input[1];
    }

    let res: usize = (0..n).fold(0, |res, i| {
        let (right, left, lower, upper): (bool, bool, bool, bool) =
            (0..n).fold((false, false, false, false), |(r, l, lo, up), j| {
                match (i, j) {
                    (i, j) if i == j => (r, l, lo, up),
                    (i, j) if x[i] < x[j] && y[i] == y[j] => (true, l, lo, up),
                    (i, j) if x[i] > x[j] && y[i] == y[j] => (r, true, lo, up),
                    (i, j) if x[i] == x[j] && y[i] > y[j] => (r, l, true, up),
                    (i, j) if x[i] == x[j] && y[i] < y[j] => (r, l, lo, true),
                    _ => (r, l, lo, up),
                }
            });

        if right && left && lower && upper {
            return res + 1;
        }

        res
    });

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
