use std::io::{stdout, BufWriter, Write};

fn dfs(x: i64, y: i64, z: &String) -> String {
    if x <= y {
        x.to_string() + z
    } else {
        dfs(x - y, y - 1, &(y.to_string() + z))
    }
}

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: i64 = read();

        let res: String = dfs(s, 9, &String::new());

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
