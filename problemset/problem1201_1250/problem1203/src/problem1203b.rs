use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let q: usize = read();

    for _ in 0..q {
        let _n: usize = read();
        let mut a: Vec<u32> = read_vec();

        a.sort();

        let g: Vec<(u32, usize)> = run_length_encoding(a);

        let l: usize = g.len();
        let h: usize = l.div_ceil(2);

        let m: u32 = g[0].0 * g[l - 1].0;
        let ok: bool = (0..h).fold(true, |acc, i| {
            if l % 2 == 1 && i == h - 1 {
                acc && g[i].0 * g[i].0 == m && g[i].1 % 4 == 0
            } else {
                acc && g[i].0 * g[l - 1 - i].0 == m && g[i].1 % 2 == 0 && g[i].1 == g[l - 1 - i].1
            }
        });

        let res: &str = if ok { "YES" } else { "NO" };

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn run_length_encoding<T: std::cmp::PartialEq + Copy>(x: Vec<T>) -> Vec<(T, usize)> {
    let mut ret: Vec<(T, usize)> = Vec::new();

    let l: usize = x.len();

    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < l {
        while j < l && x[i] == x[j] {
            j += 1usize;
        }

        ret.push((x[i], j - i));
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
