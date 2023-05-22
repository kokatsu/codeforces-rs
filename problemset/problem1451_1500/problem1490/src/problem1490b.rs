use std::io::{stdout, Write, BufWriter};

const D: usize = 3;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<usize> = read_vec();

        let mut counts: Vec<usize> = vec![0; D];
        for x in a {
            counts[x%D] += 1;
        }

        let m: usize = n / 3;
        let mut res: usize = 0;
        for _ in 0..D {
            for i in 0..D {
                if counts[i] > m {
                    let diff: usize = counts[i] - m;
                    res += diff;
                    counts[(i+1)%D] += diff;
                    counts[i] = m;
                }
            }
        }

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