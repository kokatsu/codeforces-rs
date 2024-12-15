use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let mut a: usize = 0;
        let mut l: Vec<usize> = vec![0; 26];
        for &c in s.iter() {
            let i: usize = ((c as u8) - ('a' as u8)) as usize;
            if l[i] == 0 {
                a += 1;
            }
            l[i] += 1;
        }

        let mut b: usize = 0;
        let mut r: Vec<usize> = vec![0; 26];
        let res: usize = s.iter().rev().fold(0, |res, &c| {
            let i: usize = ((c as u8) - ('a' as u8)) as usize;
            if r[i] == 0 {
                b += 1;
            }
            r[i] += 1;
            l[i] -= 1;
            if l[i] == 0 {
                a -= 1;
            }
            res.max(a + b)
        });

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

        ret.push((x[i], (j - i) as i64));
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
