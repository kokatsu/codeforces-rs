use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let mut h: Vec<i64> = read_vec();

        h.sort();

        if n == 2 {
            writeln!(out, "{} {}", h[0], h[1]).unwrap();
            continue;
        }

        let pos: usize = (1..n)
            .fold((0, i64::MAX), |(pos, diff), i| {
                if h[i] - h[i - 1] < diff {
                    (i, h[i] - h[i - 1])
                } else {
                    (pos, diff)
                }
            })
            .0;

        let s: String = (pos + 1..n).fold(h[pos].to_string(), |s, i| s + " " + &h[i].to_string());

        let res: String = (0..pos).fold(s, |res, i| res + " " + &h[i].to_string());

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
