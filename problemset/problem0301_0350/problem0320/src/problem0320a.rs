use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: String = read_string();

    let l: usize = n.len();

    let mut is_ok: bool = true;
    let mut pos: usize = 0usize;
    while pos < l {
        if pos + 3usize <= l && &n[pos..pos+3usize] == "144" {
            pos += 3usize;
        }
        else if pos + 2usize <= l && &n[pos..pos+2usize] == "14" {
            pos += 2usize;
        }
        else if &n[pos..pos+1usize] == "1" {
            pos += 1usize;
        }
        else {
            is_ok = false;
            break;
        }
    }

    let res: &str = if is_ok {"YES"} else {"NO"};

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", res).unwrap();
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