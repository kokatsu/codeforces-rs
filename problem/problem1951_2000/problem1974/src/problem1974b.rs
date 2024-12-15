use std::io::{stdout, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let b: Vec<char> = read_string().chars().collect();

        let map: HashMap<char, char> = {
            let mut c: Vec<char> = b.clone();

            c.sort();

            let u: Vec<char> = run_length_encoding(c)
                .iter()
                .map(|x| x.0)
                .collect::<Vec<char>>();
            let l: usize = u.len();

            u.clone()
                .into_iter()
                .enumerate()
                .map(|(i, x)| (x, u[l-i-1]))
                .collect::<HashMap<char, char>>()
        };

        let res: String = b
            .iter()
            .map(|x| *map.get(&x).unwrap())
            .collect();

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