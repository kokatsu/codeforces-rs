use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();

    let mut is_ok: bool = false;
    let mut seats: Vec<String> = Vec::new();
    for _ in 0..n {
        let mut s: Vec<char> = read_string().chars().collect();

        if !is_ok && s[0] == 'O' && s[1] == 'O' {
            s[0] = '+';
            s[1] = '+';
            is_ok = true;
        } else if !is_ok && s[3] == 'O' && s[4] == 'O' {
            s[3] = '+';
            s[4] = '+';
            is_ok = true;
        }

        seats.push(s.iter().collect::<String>());
    }

    let mut out = BufWriter::new(stdout().lock());

    if is_ok {
        writeln!(out, "YES").unwrap();
        writeln!(out, "{}", &seats.join("\n")).unwrap();
    } else {
        writeln!(out, "NO").unwrap();
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
