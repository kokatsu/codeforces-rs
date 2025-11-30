use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, _k): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let a: Vec<usize> = read_vec();

    let mut l: usize = 1;
    let mut res: usize = 1;
    for i in 1..n {
        if a[i] == a[i - 1] {
            l = 1;
        } else {
            l += 1;
        }
        res = res.max(l);
    }

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
