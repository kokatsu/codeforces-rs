use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let l: usize = s.len();

        let res: String = if let Some(p) = (1..l).filter(|&i| s[i - 1] == s[i]).next() {
            let c: String = (if s[p] == 'a' { 'z' } else { 'a' }).to_string();
            s[0..p].iter().collect::<String>() + &c + &s[p..l].iter().collect::<String>()
        } else {
            let c: String = (if s[l - 1] == 'a' { 'z' } else { 'a' }).to_string();
            s.iter().collect::<String>() + &c
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
