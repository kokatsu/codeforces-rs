use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let res: String =
            if n % 2 == 0 {
                "-1".to_string()
            }
            else {
                let h: usize = n / 2;
                (0..n).map(|i| {
                    let m: usize =
                        if i <= h {
                            i * 2 + 1
                        }
                        else {
                            (i - h) * 2
                        };
                    m.to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
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