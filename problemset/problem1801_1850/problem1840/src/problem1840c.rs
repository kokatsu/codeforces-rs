use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let (_n, k, q): (i64, i64, i64) = (input[0], input[1], input[2]);

        let a: Vec<i64> = {
            let mut b = read_vec();
            b.push(q + 1);
            b
        };

        let mut d: i64 = 0;
        let mut res: i64 = 0;
        for x in a.into_iter() {
            if x <= q {
                d += 1
            } else {
                let p = 0.max(d - k + 1);
                res += p * (p + 1) / 2;
                d = 0;
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
