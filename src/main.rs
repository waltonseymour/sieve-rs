use std::io::Write;

pub struct PrimeIterator {
    current: usize,
    data: Vec<bool>,
}

fn new_prime_iterator(size: usize) -> PrimeIterator {
    PrimeIterator {
        current: 2,
        data: vec![false; size],
    }
}

impl std::iter::Iterator for PrimeIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.data.len() {
            if self.data[self.current] == false {
                let mut j = self.current;

                while j < self.data.len() {
                    self.data[j] = true;
                    j += self.current;
                }
                return Some(self.current);
            }
            self.current += 1;
        }
        None
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut handle = std::io::BufWriter::new(stdout);

    let args: Vec<String> = std::env::args().collect();
    let mut size = 100;
    if args.len() > 1 {
        size = args[1].parse().unwrap();
    }

    let iter = new_prime_iterator(size);

    for prime in iter {
        writeln!(handle, "{p}", p = prime);
    }
}
