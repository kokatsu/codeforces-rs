use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<usize> = read_vec();

        let mut b: Vec<i64> = vec![i64::MIN / 2; n * 2 + 1];
        for (i, x) in a.iter().enumerate() {
            b[*x] = i as i64 + 1;
        }

        let mut res: i64 = 0;
        for i in 2..=n * 2 {
            let mut j: usize = 1;
            while j * j < i {
                if i % j == 0 && b[i / j] + b[j] == i as i64 {
                    res += 1;
                }
                j += 1;
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
