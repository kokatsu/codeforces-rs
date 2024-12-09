use std::io::{stdout, Write, BufWriter};

fn main() {
    let _n: usize = read();
    let a: Vec<usize> = read_vec();

    let mut exists: Vec<bool> = vec![false; 1001_usize];
    let mut uniques: Vec<usize> = Vec::new();
    for &x in a.iter().rev() {
        if exists[x] {
            continue;
        }

        exists[x] = true;
        uniques.push(x);
    }

    let l: usize = uniques.len();
    let elements: String =
        (0..l-1)
        .rev()
        .fold(uniques[l-1].to_string(), |elements, i| elements + " " + &uniques[i].to_string());

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}\n{}", l, elements).unwrap();
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