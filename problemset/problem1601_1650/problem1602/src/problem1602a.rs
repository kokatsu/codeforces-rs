use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let l: usize = s.len();

        let mut c: Vec<char> = s.clone();

        c.sort();

        let index: usize = (0..l).find(|&i| s[i] == c[0]).unwrap();

        let res: String = format!(
            "{} {}{}",
            c[0],
            s[0..index].iter().collect::<String>(),
            s[index + 1..l].iter().collect::<String>()
        );

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
