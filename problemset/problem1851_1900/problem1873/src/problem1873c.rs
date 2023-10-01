use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let res: usize =
            (0..10)
            .fold(0, |res, i| {
                let row: String = read_string();
                let x: usize =
                    if i < 5 {
                        i + 1
                    }
                    else {
                        10 - i
                    };
                let num: usize =
                    row
                    .chars()
                    .enumerate()
                    .fold(0, |num, (j, c)| {
                        let y: usize =
                            if j < 5 {
                                j + 1
                            }
                            else {
                                10 - j
                            };
                        if c == 'X' {
                            num + x.min(y)
                        }
                        else {
                            num
                        }
                    });
                res + num
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