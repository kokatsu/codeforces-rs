use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let mut counts: Vec<i64> = vec![0; 26];
        for x in s.chars() {
            counts[(x as usize)-('a' as usize)] += 1;
        }

        let (a, b): (i64, i64) = counts.iter()
                                        .fold((0, 0), |(a, b), &x| {
                                            if x == 1 {
                                                (a+1, b)
                                            }
                                            else if x > 1 {
                                                (a, b+1)
                                            }
                                            else {
                                                (a, b)
                                            }
                                        });

        let res: i64 = b + a / 2;

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