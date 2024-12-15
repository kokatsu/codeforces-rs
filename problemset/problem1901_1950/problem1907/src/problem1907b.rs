use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: String = read_string();

        let mut v: Vec<char> = s
            .chars()
            .rev()
            .fold((Vec::<char>::new(), 0, 0), |(mut v, lower, upper), c| {
                if c.is_lowercase() {
                    if c == 'b' {
                        (v, lower + 1, upper)
                    } else if lower > 0 {
                        (v, lower - 1, upper)
                    } else {
                        v.push(c);
                        (v, lower, upper)
                    }
                } else {
                    if c == 'B' {
                        (v, lower, upper + 1)
                    } else if upper > 0 {
                        (v, lower, upper - 1)
                    } else {
                        v.push(c);
                        (v, lower, upper)
                    }
                }
            })
            .0;

        v.reverse();

        let res: String = v.iter().collect();

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
