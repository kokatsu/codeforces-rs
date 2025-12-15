use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (l1, b1, l2, b2, l3, b3): (u16, u16, u16, u16, u16, u16) = {
            let input: Vec<u16> = read_vec();
            (input[0], input[1], input[2], input[3], input[4], input[5])
        };

        let res: &str = if (l1 + l2 + l3 == b1 && b1 == b2 && b2 == b3)
            || (l1 == l2 + l3 && b1 + b2 == l1 && b2 == b3)
            || (b1 + b2 + b3 == l1 && l1 == l2 && l2 == l3)
            || (b1 == b2 + b3 && l1 + l2 == b1 && l2 == l3)
        {
            "YES"
        } else {
            "NO"
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
