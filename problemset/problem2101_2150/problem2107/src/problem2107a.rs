use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u16> = read_vec();

        let m: u16 = *a.iter().max().unwrap_or(&0);

        let res: String = if a.iter().any(|&x| x != m) {
            let group: Vec<u16> = a.iter().map(|&x| if x == m { 1 } else { 2 }).collect();
            format!(
                "Yes\n{}",
                group
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else {
            "No".to_string()
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
