pub struct Fibonacci {
    a: u64,
    b: u64,
    curr: u8,
    total: u8,
}

impl Fibonacci {
    pub fn new(total: u8) -> Self {
        Self {
            a: 0,
            b: 0,
            curr: 0,
            total,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.total {
            return None;
        }

        if self.a == 0 {
            self.a = 1;
            self.b = 1;
        } else {
            let c = self.a + self.b;
            self.a = self.b;
            self.b = c;
        }

        self.curr += 1;
        Some(self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::Fibonacci;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new(10);
        assert_eq!(fib.next(), Some(1));

        for (i, item) in fib.into_iter().enumerate() {
            println!("{}: {}", i, item);
        }
    }
}
