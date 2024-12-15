use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let yes: String = "Yes".to_string().repeat(20);

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let res: &str =
            if yes.contains(&s) {
                "YES"
            }
            else {
                "NO"
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