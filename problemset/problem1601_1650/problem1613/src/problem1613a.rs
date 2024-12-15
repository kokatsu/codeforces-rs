use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x1, p1): (u64, u32) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1] as u32)
        };

        let (x2, p2): (u64, u32) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1] as u32)
        };

        let m: u32 = p1.min(p2);
        let q1: u32 = p1 - m;
        let q2: u32 = p2 - m;

        let res: &str = if q1 > 6 {
            ">"
        } else if q2 > 6 {
            "<"
        } else {
            let y1: u64 = x1 * 10_u64.pow(q1);
            let y2: u64 = x2 * 10_u64.pow(q2);
            if y1 > y2 {
                ">"
            } else if y1 < y2 {
                "<"
            } else {
                "="
            }
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
