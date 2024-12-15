use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let (min, max, neq): (u64, u64, Vec<u64>) = (0..n).fold(
            (0, u64::MAX, Vec::<u64>::new()),
            |(min, max, mut neq), _| {
                let input: Vec<u64> = read_vec();
                let (a, x): (u64, u64) = (input[0], input[1]);
                match a {
                    1 => (min.max(x), max, neq),
                    2 => (min, max.min(x), neq),
                    _ => {
                        neq.push(x);
                        (min, max, neq)
                    }
                }
            },
        );

        let count: u64 = neq.into_iter().filter(|&x| min <= x && x <= max).count() as u64;

        let res: u64 = if max >= min + count {
            max - min - count + 1
        } else {
            0
        };

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
