use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (_n, _m, k): (usize, usize, usize) = (input[0], input[1], input[2]);

        let a: Vec<usize> = read_vec();
        let b: Vec<usize> = read_vec();

        let mut c: Vec<u8> = vec![0; k];

        for x in a.into_iter() {
            if x - 1 < k {
                c[x-1] |= 1;
            }
        }

        for x in b.into_iter() {
            if x - 1 < k {
                c[x-1] |= 2;
            }
        }

        let (u, v, ok): (usize, usize, bool) = c
            .iter()
            .fold((0, 0, true), |(u, v, ok), x| {
                match x {
                    0 => (u, v, false),
                    1 => (u+1, v, ok),
                    2 => (u, v+1, ok),
                    _ => (u, v, ok),
                }
            });

        let res: &str =
            if u.max(v) <= k / 2 && ok {
                "YES"
            }
            else {
                "NO"
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