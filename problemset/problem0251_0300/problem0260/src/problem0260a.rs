use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (a, b, n): (u32, u32, u32) = {
        let input = read_vec::<u32>();
        (input[0], input[1], input[2])
    };

    let p: Option<u32> = (0..10).find(|i| (a * 10 + i) % b == 0);

    let res: String = if let Some(p) = p {
        let z: String = "0".repeat((n - 1) as usize);
        format!("{}{}{}", a, p, z)
    } else {
        "-1".to_string()
    };

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
