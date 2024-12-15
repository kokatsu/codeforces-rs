use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut list: Vec<u64> = Vec::new();
    let mut num: u64 = 2050;
    let upper: u64 = 10_u64.pow(18);
    while num <= upper {
        list.push(num);
        num *= 10;
    }
    list.reverse();

    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: u64 = read();

        let (cnt, rem): (u64, u64) = list
            .iter()
            .fold((0, n), |(cnt, rem), x| (cnt + rem / x, rem % x));

        let res: i64 = if rem == 0 { cnt as i64 } else { -1 };

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
