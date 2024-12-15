use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();

        let mut minutes: Vec<i64> = vec![i64::MAX / 2; 4];
        for _ in 0..n {
            let input: Vec<String> = read_vec();
            let m: i64 = input[0].parse().ok().unwrap();
            let s: usize = usize::from_str_radix(&input[1], 2).unwrap();

            minutes[s] = minutes[s].min(m);
        }

        let num: i64 = minutes[3].min(minutes[1] + minutes[2]);

        let res: i64 = if num < i64::MAX / 2 { num } else { -1 };

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
