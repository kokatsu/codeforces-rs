use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let (x, y): (u32, u32) = (0..n)
            .fold((0, 0), |(x, y), _| {
                let (w, h): (u32, u32) = {
                    let input: Vec<u32> = read_vec();
                    (input[0], input[1])
                };
                (x.max(w), y.max(h))
            });

        let res: u32 = (x + y) * 2;

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