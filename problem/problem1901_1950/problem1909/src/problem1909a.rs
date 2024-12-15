use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let mut v: Vec<i8> = vec![0; 4];

        for _ in 0..n {
            let (x, y): (i8, i8) = {
                let input: Vec<i8> = read_vec();
                (input[0], input[1])
            };

            if x > 0 {
                v[1] = 1;
            }
            else if x < 0 {
                v[0] = 1;
            }

            if y > 0 {
                v[3] = 1;
            }
            else if y < 0 {
                v[2] = 1;
            }
        }

        let res: &str = if v.iter().sum::<i8>() < 4 { "YES" } else { "NO" };

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