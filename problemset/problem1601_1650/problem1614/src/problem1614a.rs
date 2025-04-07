use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, l, r, mut k): (usize, u32, u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0] as usize, input[1], input[2], input[3])
        };

        let mut a: Vec<u32> = read_vec();

        a.sort();

        let mut res: usize = 0;
        for &x in a.iter().filter(|&&x| x >= l && x <= r) {
            if k >= x {
                k -= x;
                res += 1;
            } else {
                break;
            }
        }

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
