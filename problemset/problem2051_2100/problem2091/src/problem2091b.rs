use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, x): (usize, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0] as usize, input[1])
        };

        let mut a: Vec<u64> = read_vec();

        a.sort_by(|x, y| x.cmp(y).reverse());

        let res: u64 = a
            .iter()
            .fold((0, 0), |(res, count), &u| {
                if u * (count + 1) >= x {
                    (res + 1, 0)
                } else {
                    (res, count + 1)
                }
            })
            .0;

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
