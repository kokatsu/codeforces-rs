use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();

        let (mihai, bianca): (i64, i64) = a.iter()
                                            .fold((0, 0), |(mihai, bianca), x| {
                                                if x % 2 == 0 {
                                                    (mihai+x, bianca)
                                                }
                                                else {
                                                    (mihai, bianca+x)
                                                }
                                            });

        let res: &str =
            if mihai > bianca {
                "YES"
            }
            else {
                "NO"
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