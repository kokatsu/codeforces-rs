use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<i64> = read_vec();

        let b: Vec<i64> = (0..n)
            .map(|_| {
                let input: Vec<String> = read_vec();
                input[1].chars().fold(0, |num, x| {
                    if x == 'U' {
                        if num == 0 {
                            9
                        } else {
                            num - 1
                        }
                    } else {
                        num + 1
                    }
                })
            })
            .collect();

        let res: String = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x + y) % 10)
            .fold(String::new(), |res, x| res + &x.to_string() + " ");

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
