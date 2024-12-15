use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let a: i64 = input[0];
        let b: i64 = input[1];
        let x: i64 = input[2];
        let y: i64 = input[3];
        let n: i64 = input[4];

        let ma: i64 = x.max(a-n);
        let na: i64 = n - a + ma;
        let u: i64 = ma * y.max(b-na);

        let mb: i64 = y.max(b-n);
        let nb: i64 = n - b + mb;
        let v: i64 = mb * x.max(a-nb);

        let res: i64 = u.min(v);

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