use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let d: usize = input[1];

        let s: String = read_string();

        let v: Vec<usize> = s.chars().map(|c| c as usize - '0' as usize).collect();
        let p: Option<usize> = v.iter().position(|&x| x < d);

        let res: String =
            if p == None {
                s + &d.to_string()
            }
            else {
                s[0..p.unwrap()].to_string() + &d.to_string() + &s[p.unwrap()..n]
            };

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