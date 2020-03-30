pub struct PrimeIterator {
    current: usize,
    data: std::vec::Vec<bool>,
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
    let iter = new_prime_iterator(10000);

    for prime in iter {
        println!("{p}", p = prime);
    }
}
