use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let mut s: Vec<char> = read_string().chars().collect();

    let mut c: usize = 0;
    for i in 0..n / 2 {
        if s[i * 2] == 'a' && s[i * 2 + 1] == 'a' {
            s[i * 2] = 'b';
            c += 1;
        } else if s[i * 2] == 'b' && s[i * 2 + 1] == 'b' {
            s[i * 2] = 'a';
            c += 1;
        }
    }

    let t: String = s.iter().collect();

    let res: String = format!("{}\n{}", c, t);

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
