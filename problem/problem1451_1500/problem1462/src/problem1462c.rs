use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let mut x: i64 = read();

        let mut used: Vec<bool> = vec![false; 10];
        let mut p: i64 = 1;
        let mut res: i64 = 0;
        while x > 0 {
            let mut is_ok: bool = false;
            for i in (1..10).rev() {
                if used[i] || x < (i as i64) {
                    continue;
                }

                is_ok = true;
                x -= i as i64;
                used[i] = true;
                res += i as i64 * p;
                p *= 10;
                break;
            }

            if !is_ok {
                res = -1;
                break;
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