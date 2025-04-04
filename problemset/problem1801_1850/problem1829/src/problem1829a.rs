use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let x: Vec<char> = "codeforces".chars().collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let res: i64 = s
            .iter()
            .zip(x.iter())
            .fold(0, |res, (a, b)| if a == b { res } else { res + 1 });

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
