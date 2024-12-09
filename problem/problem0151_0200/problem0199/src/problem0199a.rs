use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: u64 = read();

    let mut fib: Vec<u64> = vec![0, 1];
    for i in 1..100 {
        if fib[i] > n {
            break;
        }

        fib.push(fib[i - 1] + fib[i]);
    }

    let l: usize = fib.len();
    for i in 0..l {
        for j in i..l {
            for k in j..l {
                if fib[i] + fib[j] + fib[k] == n {
                    writeln!(out, "{} {} {}", fib[i], fib[j], fib[k]).unwrap();
                    return;
                }
            }
        }
    }

    writeln!(out, "I'm too stupid to solve this problem").unwrap();
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
