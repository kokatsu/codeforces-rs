use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<char> = read_string().chars().collect();
        let _m: usize = read();
        let b: Vec<char> = read_string().chars().collect();
        let c: String = read_string();

        let mut v: Vec<char> = Vec::new();
        let mut d: Vec<char> = Vec::new();

        for (i, x) in c.chars().enumerate() {
            if x == 'V' {
                v.push(b[i]);
            } else {
                d.push(b[i]);
            }
        }

        v.reverse();

        let res: String = v.into_iter().collect::<String>()
            + &a.into_iter().collect::<String>()
            + &d.into_iter().collect::<String>();

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
