use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (b, k): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let a: Vec<usize> = read_vec();

    let c: usize = a.iter().filter(|&&x| x % 2 == 1).count();

    let res: &str = if b % 2 == 0 {
        if a[k - 1] % 2 == 0 {
            "even"
        } else {
            "odd"
        }
    } else {
        if c % 2 == 0 {
            "even"
        } else {
            "odd"
        }
    };

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
