use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u8> = read_vec();

        let res: i16 = (0..n).fold(1, |acc, i| {
            if acc == -1 {
                return -1;
            }
            if a[i] == 0 {
                if i > 0 && a[i - 1] == 0 {
                    -1
                } else {
                    acc
                }
            } else if i > 0 && a[i - 1] == 1 {
                acc + 5
            } else {
                acc + 1
            }
        });

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
