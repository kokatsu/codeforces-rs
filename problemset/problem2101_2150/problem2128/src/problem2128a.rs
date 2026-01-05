use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, c): (usize, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0] as usize, input[1])
        };

        let mut a: Vec<u64> = read_vec();

        a.sort();

        let res: u64 = a
            .iter()
            .rev()
            .fold((0, 1), |(res, mul), &x| {
                if mul * x > c {
                    (res + 1, mul)
                } else {
                    (res, mul * 2)
                }
            })
            .0;

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
