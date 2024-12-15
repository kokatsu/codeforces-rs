use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let res: String = match n % 3 {
            1 => [1, 2]
                .iter()
                .cycle()
                .take(n / 3 * 2 + 1)
                .map(|x| x.to_string())
                .collect::<String>(),
            2 => [2, 1]
                .iter()
                .cycle()
                .take(n / 3 * 2 + 1)
                .map(|x| x.to_string())
                .collect::<String>(),
            _ => [2, 1]
                .iter()
                .cycle()
                .take(n / 3 * 2)
                .map(|x| x.to_string())
                .collect::<String>(),
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
