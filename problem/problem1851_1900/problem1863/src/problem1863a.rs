use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, a, _q): (i32, i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1], input[2])
        };

        let s: String = read_string();

        let (plus, high, _current): (i32, i32, i32) = s
            .chars()
            .fold((0, a, a), |(plus, high, current), x| {
                let (p, c): (i32, i32) =
                    if x == '+' {
                        (plus + 1, current + 1)
                    }
                    else {
                        (plus, current - 1)
                    };
                (p, high.max(c), c)
            });

        let res: &str =
            if a + plus < n {
                "NO"
            }
            else if n - high <= 0 {
                "YES"
            }
            else {
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