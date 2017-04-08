use std::io;

mod iterator_extensions;
mod numerics;

use iterator_extensions::IteratorExtensions;
use numerics::BigInteger;


fn main() {

    let mut fib = Fib::new().memoize();

    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();

        match trimmed.parse::<u32>() {
            Ok(i) => {
                let mut fib1 = &mut fib;
                let x = fib1.into_iter().skip(i as usize).next().unwrap();
                println!("{} : {} ({})", i, x.to_string(), x.get_code());
            }
            Err(..) => break,
        };
    }
}


struct Fib {
    is_zero: bool,
    z2: BigInteger,
    z1: BigInteger,
}

impl Fib {
    pub fn new() -> Fib {
        Fib {
            is_zero: true,
            z2: BigInteger::one(),
            z1: BigInteger::zero(),
        }
    }
}
impl Iterator for Fib {
    type Item = BigInteger;

    fn next(&mut self) -> Option<BigInteger> {
        if self.is_zero {
            self.is_zero = false;
            Some(BigInteger::zero())
        } else {
            let r = &self.z2 + &self.z1;
            self.z2 = self.z1.clone();
            self.z1 = r.clone();
            println!("next : {}", r.to_string());
            Some(r)
        }
    }
}
