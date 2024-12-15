use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let k: u64 = input[1] as u64;

        let mut a: Vec<u64> = read_vec();

        a.sort();

        let mut num: usize = 1;
        let mut cnt: usize = 1;
        for i in 1..n {
            if a[i] - a[i - 1] > k {
                cnt = 1;
            } else {
                cnt += 1;
                num = num.max(cnt);
            }
        }

        let res: usize = n - num.max(cnt);

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
