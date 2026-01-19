use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a1, b1): (u8, u8) = {
            let input: Vec<u8> = read_vec();
            (input[0], input[1])
        };

        let (a2, b2): (u8, u8) = {
            let input: Vec<u8> = read_vec();
            (input[0], input[1])
        };

        let res: &str = if (a1 == a2 && a1 == b1 + b2)
            || (a1 == b2 && a1 == b1 + a2)
            || (b1 == a2 && b1 == a1 + b2)
            || (b1 == b2 && b1 == a1 + a2)
        {
            "Yes"
        } else {
            "No"
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
