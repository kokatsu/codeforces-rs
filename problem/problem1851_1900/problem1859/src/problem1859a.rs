use std::io::{stdout, BufWriter, Write};

fn make_result(v: &[u64], m: usize, n: usize) -> String {
    let counts: String = m.to_string() + " " + &(n - m).to_string();
    let b: String = (1..m).fold(v[0].to_string(), |b, i| b + " " + &v[i].to_string());
    let c: String = (m + 1..n).fold(v[m].to_string(), |c, i| c + " " + &v[i].to_string());
    counts + "\n" + &b + "\n" + &c
}

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<u64> = read_vec();

        a.sort();

        let index: Option<usize> = a.iter().position(|&x| x != a[0]);

        let res: String = match index {
            Some(index) => make_result(&a, index, n),
            None => (-1).to_string(),
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
