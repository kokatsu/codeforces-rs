use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u32> = read_vec();
        let (zero, one, two): (usize, usize, usize) =
            a.iter().fold((0, 0, 0), |(z, o, t), &x| match x % 3 {
                0 => (z + 1, o, t),
                1 => (z, o + 1, t),
                _ => (z, o, t + 1),
            });

        let m: usize = one.min(two);

        let res: usize = zero + m + (one - m) / 3 + (two - m) / 3;

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
