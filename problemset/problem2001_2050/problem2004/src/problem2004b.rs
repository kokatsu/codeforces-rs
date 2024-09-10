use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (la, ra): (u8, u8) = {
            let input: Vec<u8> = read_vec();
            (input[0], input[1])
        };

        let (lb, rb): (u8, u8) = {
            let input: Vec<u8> = read_vec();
            (input[0], input[1])
        };

        let (l, r): (u8, u8) = (la.max(lb), ra.min(rb));

        let res: u8 =
            if l > r {
                1
            }
            else {
                r - l + (la != lb) as u8 + (ra != rb) as u8
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