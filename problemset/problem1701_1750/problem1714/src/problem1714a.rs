use std::io::{stdout, BufWriter, Write};

const H: usize = 24;
const M: usize = 60;
const L: usize = H * M;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let t: usize = input[1] * M + input[2];

        let time: usize = (0..n).fold(L, |time, _| {
            let alarm: Vec<usize> = read_vec();
            time.min((alarm[0] * M + alarm[1] + L - t) % L)
        });

        let h: usize = time / M;
        let m: usize = time % M;

        writeln!(out, "{} {}", h, m).unwrap();
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
