use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let a: String = (0..n * 2)
            .map(|i| if i % 4 < 2 { '#' } else { '.' })
            .collect();
        let b: String = (0..n * 2)
            .map(|i| if i % 4 < 2 { '.' } else { '#' })
            .collect();

        let res: String = (0..n * 2)
            .map(|i| if i % 4 < 2 { a.clone() } else { b.clone() })
            .collect::<Vec<String>>()
            .join("\n");

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
