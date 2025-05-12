use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b, c, d, k): (u8, u8, u8, u8, u8) = {
            let input: Vec<u8> = read_vec();
            (input[0], input[1], input[2], input[3], input[4])
        };

        let x: u8 = a.div_ceil(c);
        let y: u8 = b.div_ceil(d);

        let res: String = if x + y <= k {
            format!("{} {}", x, y)
        } else {
            "-1".to_string()
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
