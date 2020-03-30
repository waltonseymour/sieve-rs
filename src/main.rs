pub struct PrimeReader {
    current: usize,
    data: std::vec::Vec<bool>,
}

impl std::io::Read for PrimeReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        while self.current < self.data.len() {
            if self.data[self.current] == false {
                //buf.
            }
        }

        Ok(2)
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        Ok(2)
    }
}

fn main() {
    // primes(100)
}
