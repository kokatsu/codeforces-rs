use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, _m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let (x, y, _num): (usize, usize, usize) = (1..=n).fold((0, 0, 0), |(x, y, num), i| {
            let s: Vec<char> = read_string().chars().collect();
            let g: Vec<(char, i64)> = run_length_encoding(s);
            let mut v: usize = 0;
            let mut cnt: usize = 0;
            for (c, d) in g.into_iter() {
                let u: usize = d as usize;
                if c == '#' {
                    v += (u + 1) / 2;
                    cnt = u;
                    break;
                } else {
                    v += u;
                }
            }
            if cnt > num {
                (i, v, cnt)
            } else {
                (x, y, num)
            }
        });

        let res: String = format!("{} {}", x, y);

        writeln!(out, "{}", res).unwrap();
    }
}

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
