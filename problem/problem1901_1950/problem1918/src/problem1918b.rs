use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let a: Vec<u32> = read_vec();
        let b: Vec<u32> = read_vec();

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&i, &j| a[i].cmp(&a[j]));

        let x: String = indices
            .iter()
            .map(|&i| a[i].to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let y: String = indices
            .iter()
            .map(|&i| b[i].to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let res: String = x + "\n" + &y;

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