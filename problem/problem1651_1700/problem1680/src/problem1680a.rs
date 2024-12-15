use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (l1, r1, l2, r2): (u8, u8, u8, u8) = {
            let input: Vec<u8> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let res: u8 = if l1 <= l2 && l2 <= r1 {
            l2
        } else if l2 <= l1 && l1 <= r2 {
            l1
        } else {
            l1 + l2
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
