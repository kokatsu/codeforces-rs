use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let n: i64 = input[0];
        let _m: i64 = input[1];
        let x: i64 = input[2];
        let y: i64 = input[3];

        let res: i64 =
            (0..n)
            .fold(0, |res, _| {
                let a: Vec<char> = read_string().chars().collect();
                let g: Vec<(char, i64)> = run_length_encoding(a);
                let num: i64 =
                    g
                    .iter()
                    .fold(0, |num, (u, v)| {
                        if *u == '.' {
                            num + v / 2 * y.min(x*2) + v % 2 * x
                        }
                        else {
                            num
                        }
                    });
                res + num
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