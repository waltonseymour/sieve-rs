use std::io::Write;

fn primes(n: usize) {
    let mut stdout = std::io::stdout();
    let mut data = vec![false; n];

    for i in 2..n {
        if data[i] == false {
            writeln!(stdout, "{prime}", prime = i);
            let mut j = i;
            while j < n {
                data[j] = true;
                j += i;
            }
        }
    }
}

fn main() {
    primes(100)
}
