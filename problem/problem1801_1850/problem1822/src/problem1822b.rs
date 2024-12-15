use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();

        let mut pos: Vec<i64> = Vec::new();
        let mut neg: Vec<i64> = Vec::new();
        let mut has0: bool = false;
        for x in a.into_iter() {
            if x > 0 {
                pos.push(x);
            } else if x < 0 {
                neg.push(x);
            } else {
                has0 = true;
            }
        }

        pos.sort_by(|x, y| x.cmp(y).reverse());
        neg.sort();

        let p: i64 = if pos.len() > 1 {
            pos[0] * pos[1]
        } else {
            i64::MIN
        };
        let q: i64 = if neg.len() > 1 {
            neg[0] * neg[1]
        } else {
            i64::MIN
        };
        let r: i64 = if has0 { 0 } else { i64::MIN };
        let s: i64 = if !pos.is_empty() && !neg.is_empty() {
            pos.iter().last().unwrap() * neg.iter().last().unwrap()
        } else {
            i64::MIN
        };

        let res: i64 = p.max(q).max(r).max(s);

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
