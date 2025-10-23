use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: usize = read();
    let s: Vec<char> = read_string().chars().collect();

    let g: Vec<(char, usize)> = run_length_encoding(s);

    let b: Vec<usize> = g
        .iter()
        .filter(|&&(c, _)| c == 'B')
        .map(|&(_, cnt)| cnt)
        .collect();

    let res: String = if b.is_empty() {
        0.to_string()
    } else {
        format!(
            "{}\n{}",
            b.len(),
            b.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
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
