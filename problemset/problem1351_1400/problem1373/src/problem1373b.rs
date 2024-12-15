use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: i64 = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let l: usize = s.len();
        let m: usize = s.chars().filter(|&x| x == '0').collect::<Vec<_>>().len();
        let n: usize = m.min(l - m);

        let res: &str = if n % 2 == 1 { "DA" } else { "NET" };

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
