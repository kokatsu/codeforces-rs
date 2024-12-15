use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let _n: usize = input[0];
        let k: usize = input[1];

        let a: Vec<usize> = read_vec();
        let b: Vec<usize> = read_vec();

        let mut c: Vec<(usize, usize)> = a
            .into_iter()
            .zip(b.into_iter())
            .collect::<Vec<(usize, usize)>>();

        c.sort_by(|a, b| a.0.cmp(&b.0));

        let res: usize = c
            .iter()
            .fold(k, |res, x| if res >= x.0 { res + x.1 } else { res });

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
