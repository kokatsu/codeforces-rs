use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let n: u64 = input[0];
        let s: u64 = input[1];
        let r: u64 = input[2];

        let max: u64 = s - r;

        let res: String = (1..n)
            .fold((max.to_string(), r), |(res, rem), i| {
                let num = max.min(rem + i + 1 - n);
                (res + " " + &num.to_string(), rem - num)
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
