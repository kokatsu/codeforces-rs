use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<usize> = read_vec();

        let mut mods: [usize; 10] = [0; 10];
        for &x in &a {
            mods[x % 10] += 1;
        }

        let mut ok: bool = false;
        for i in 0..10 {
            if mods[i] == 0 {
                continue;
            }

            mods[i] -= 1;

            for j in 0..10 {
                if mods[j] == 0 {
                    continue;
                }

                mods[j] -= 1;

                #[allow(clippy::needless_range_loop)]
                for k in 0..10 {
                    if mods[k] == 0 {
                        continue;
                    }

                    if (i + j + k) % 10 == 3 {
                        ok = true;
                        break;
                    }
                }

                mods[j] += 1;

                if ok {
                    break;
                }
            }

            mods[i] += 1;

            if ok {
                break;
            }
        }

        let res: &str = if ok { "YES" } else { "NO" };

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
