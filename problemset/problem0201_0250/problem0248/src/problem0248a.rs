use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let (l0, l1, r0, r1): (u16, u16, u16, u16) =
        (0..n).fold((0, 0, 0, 0), |(l0, l1, r0, r1), _| {
            let (l, r): (u16, u16) = {
                let input: Vec<u16> = read_vec();
                (input[0], input[1])
            };
            match (l, r) {
                (0, 1) => (l0 + 1, l1, r0, r1 + 1),
                (1, 0) => (l0, l1 + 1, r0 + 1, r1),
                (1, 1) => (l0, l1 + 1, r0, r1 + 1),
                _ => (l0 + 1, l1, r0 + 1, r1),
            }
        });

    let res: u16 = l0.min(l1) + r0.min(r1);

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
