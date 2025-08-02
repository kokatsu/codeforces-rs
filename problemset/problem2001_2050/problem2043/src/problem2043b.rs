use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, d): (u32, u32) = {
            let input = read_vec::<u32>();
            (input[0], input[1])
        };

        let mut odds: Vec<String> = Vec::new();
        odds.push("1".to_string());

        if d % 3 == 0 || n >= 3 {
            odds.push("3".to_string());
        }

        if d % 5 == 0 {
            odds.push("5".to_string());
        }

        if d % 7 == 0 || n >= 3 {
            odds.push("7".to_string());
        }

        if d % 9 == 0 || (d % 3 == 0 && n >= 3) || n >= 6 {
            odds.push("9".to_string());
        }

        let res: String = odds.join(" ");

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
