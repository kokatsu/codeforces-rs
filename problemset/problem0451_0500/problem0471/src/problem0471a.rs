use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let mut l: Vec<u8> = read_vec();

    l.sort();

    let g: Vec<(u8, usize)> = run_length_encoding(l);

    let res = if g.iter().any(|(_, cnt)| *cnt >= 4) {
        let leg_idx = g.iter().position(|(_, cnt)| *cnt >= 4).unwrap();
        let leg_len = g[leg_idx].0;

        let mut others: Vec<u8> = Vec::new();
        for (val, cnt) in &g {
            if *val == leg_len {
                for _ in 0..(cnt - 4) {
                    others.push(*val);
                }
            } else {
                for _ in 0..*cnt {
                    others.push(*val);
                }
            }
        }

        let head = others[0];
        let body = others[1];

        if head < body {
            "Bear"
        } else {
            "Elephant"
        }
    } else {
        "Alien"
    };

    writeln!(out, "{}", res).unwrap();
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

        ret.push((x[i], (j - i)));
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
