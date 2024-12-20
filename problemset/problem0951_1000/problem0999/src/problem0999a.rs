use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, k): (usize, u16) = {
        let input = read_vec::<usize>();
        (input[0], input[1] as u16)
    };

    let a: Vec<u16> = read_vec();

    let l: usize = a.iter().position(|&x| x > k).unwrap_or(n);
    let r: usize = a.iter().rposition(|&x| x > k).unwrap_or(0);

    let res: usize = if l <= r { l + n - r - 1 } else { n };

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
