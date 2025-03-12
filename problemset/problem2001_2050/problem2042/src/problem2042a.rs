use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, k): (usize, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0] as usize, input[1])
        };

        let mut a: Vec<u32> = read_vec();

        let s: u32 = a.iter().sum();

        let res: u32 = if s >= k {
            a.sort_by(|x, y| x.cmp(y).reverse());

            let mut m: u32 = 0;
            let mut i: usize = 0;
            while i < n {
                m += a[i];

                if m >= k {
                    break;
                }

                i += 1;
            }

            (a[i] - (m - k)) % a[i]
        } else {
            k - s
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
