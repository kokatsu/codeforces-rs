use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: i64 = read();

        let c: i64 = (n as f64).cbrt().floor() as i64;

        let mut is_ok: bool = false;
        for i in 2..=c {
            if is_ok {
                break;
            }

            for j in i+1..=n {
                let k: i64 = n / (i * j);
                if k <= j {
                    break;
                }

                if i * j * k == n {
                    is_ok = true;
                    writeln!(out, "YES").unwrap();
                    writeln!(out, "{} {} {}", i, j, k).unwrap();
                    break;
                }
            }
        }

        if !is_ok {
            writeln!(out, "NO").unwrap();
        }
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