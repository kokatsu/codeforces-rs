use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let mut a: Vec<usize> = read_vec();

        a.sort();

        let g: Vec<(usize, i64)> = run_length_encoding(a);
        let l: usize = g.len();

        let res: i64 =
            (0..l)
            .flat_map(|i| (i..l).map(move |j| (i, j)))
            .fold((0, vec![0; 101]), |(res, mut cnts), (i, j)| {
                if i == j {
                    cnts[g[i].0+g[j].0] += g[i].1 / 2;
                }
                else {
                    cnts[g[i].0+g[j].0] += (g[i].1).min(g[j].1);
                }
                (res.max(cnts[g[i].0+g[j].0]), cnts)
            }).0;

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn run_length_encoding<T: std::cmp::PartialEq + Copy>(x: Vec<T>) -> Vec<(T, i64)> {
    let mut ret: Vec<(T, i64)> = Vec::new();

    let l: usize = x.len();

    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < l {
        while j < l && x[i] == x[j] {
            j += 1usize;
        }

        ret.push((x[i], (j-i) as i64));
        i = j;
    }

    ret
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