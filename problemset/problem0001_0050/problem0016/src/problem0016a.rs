use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, _m): (usize, usize) = {
        let input = read_vec::<usize>();
        (input[0], input[1])
    };

    let mut p: char = ' ';
    let mut ok: bool = true;
    for _ in 0..n {
        let c: Vec<char> = read_string().chars().collect();
        if !ok {
            continue;
        }

        let g: Vec<(char, usize)> = run_length_encoding(c);

        if g.len() > 1 {
            ok = false;
        }

        if g[0].0 == p {
            ok = false;
        }

        p = g[0].0;
    }

    let res: &str = if ok { "YES" } else { "NO" };

    writeln!(out, "{}", res).unwrap();
}

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
