use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let mut m: Vec<Vec<u64>> = vec![vec![0; n]; n];
    for x in m.iter_mut() {
        let row: Vec<u64> = read_vec();
        for (i, v) in x.iter_mut().enumerate() {
            *v = row[i];
        }
    }

    let d: u64 = ((m[0][1] * m[0][2] / m[1][2]) as f64).sqrt().floor() as u64;

    let res: String = d.to_string()
        + " "
        + &(1..n)
            .map(|i| (m[0][i] / d).to_string())
            .collect::<Vec<String>>()
            .join(" ");

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
