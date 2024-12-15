use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: String = read_string();

        let v: Vec<i64> = n.chars().into_iter().map(|x| x as i64 - 48).collect();
        let l: usize = v.len();

        let res: i64 = if v[l - 1] % 2 == 0 {
            0
        } else if v[0] % 2 == 0 {
            1
        } else if v.iter().any(|x| x % 2 == 0) {
            2
        } else {
            -1
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
