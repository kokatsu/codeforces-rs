use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u8> = read_vec();

        let zero: Option<usize> = (0..n).filter(|&i| a[i] == 0).nth(2);
        let one: Option<usize> = (0..n).find(|&i| a[i] == 1);
        let two: Option<usize> = (0..n).filter(|&i| a[i] == 2).nth(1);
        let three: Option<usize> = (0..n).find(|&i| a[i] == 3);
        let five: Option<usize> = (0..n).find(|&i| a[i] == 5);

        let res: usize = match (zero, one, two, three, five) {
            (Some(z), Some(o), Some(t), Some(th), Some(f)) => z.max(o).max(t).max(th).max(f) + 1,
            _ => 0,
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
