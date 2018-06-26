pub fn nth(n: u64) -> Option<u64> {
    factorize(n).find()
}

use std::iter::Peekable;

pub struct Primes {
    primes: Vec<u64>,
    current: u64,
}

pub fn primes() -> Primes {
    Primes {
        primes: vec![],
        current: 2,
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        for i in self.current..u64::max_value() {
            if self.primes.iter().all(|x| i % x != 0) {
                self.primes.push(i);
                self.current = i+1;
                return Some(i);
            }
        }

        panic!("Integer overflowed!")
    }
}

pub struct Factorize {
    n: u64,
    primes: Peekable<Primes>,
}

impl Iterator for Factorize {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match (self.n, self.primes.peek().cloned()) {
            (1, _) => None,

            (m, Some(p)) => {
                if m < p * p {
                    self.n = 1;
                    return Some(m);
                }

                let (q, r) = (m / p, m % p);

                if r == 0 {
                    self.n = q;
                    Some(p)
                } else {
                    self.primes.next();
                    self.next()
                }
            }

            (_, None) => {
                unreachable!()
            }
        }
    }
}

fn factorize(n: u64) -> Factorize {
    Factorize {
        n,
        primes: primes().peekable(),
    }
}
