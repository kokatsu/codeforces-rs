use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let k: usize = read();
        let mut a: Vec<usize> = read_vec();

        a.sort();

        let n: usize = k - 2;
        for (i, x) in a.iter().enumerate().rev() {
            if n % x != 0 {
                continue;
            }

            let m: usize = n / x;
            if let Ok(j) = a[0..i].binary_search(&m) {
                let res: String = a[j].to_string() + " " + &x.to_string();

                writeln!(out, "{}", res).unwrap();

                break;
            }
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
