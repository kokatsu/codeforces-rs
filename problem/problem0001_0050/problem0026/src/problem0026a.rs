use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();

    let mut counts: Vec<u64> = vec![0; n + 1];
    for i in 2..=n {
        if counts[i] == 0 {
            for j in (i * 2..=n).step_by(i) {
                counts[j] += 1;
            }
        }
    }

    let res: usize = counts.iter().filter(|&&x| x == 2).count();

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", res).unwrap();
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
