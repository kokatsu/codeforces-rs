use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, f, k): (usize, usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1] - 1, input[2])
        };

        let a: Vec<u32> = read_vec();

        let fav: u32 = a[f];
        let upper: usize = a.clone().into_iter().filter(|&x| x > fav).count();
        let equal: usize = a.into_iter().filter(|&x| x == fav).count();

        let res: &str = if k >= equal + upper {
            "YES"
        } else if k <= upper {
            "NO"
        } else {
            "MAYBE"
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
