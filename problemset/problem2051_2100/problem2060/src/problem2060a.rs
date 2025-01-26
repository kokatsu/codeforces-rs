use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let mut a: Vec<i16> = {
            let input: Vec<i16> = read_vec();

            let mut a: Vec<i16> = vec![0; 5];
            a[0] = input[0];
            a[1] = input[1];
            a[3] = input[2];
            a[4] = input[3];

            a
        };

        let list: Vec<i16> = vec![a[0] + a[1], a[3] - a[1], a[4] - a[3]];

        let res: usize = list.iter().fold(0, |res, &x| {
            a[2] = x;
            (2..5)
                .filter(|i| a[i - 2] + a[i - 1] == a[*i])
                .count()
                .max(res)
        });

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
