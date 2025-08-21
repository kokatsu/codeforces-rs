use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let a: usize = n.div_ceil(2);
    let b: usize = n - a;
    let c: usize = a * a + b * b;

    let s: String = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| if j % 2 == i % 2 { 'C' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    let res: String = format!("{}\n{}", c, s);

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
