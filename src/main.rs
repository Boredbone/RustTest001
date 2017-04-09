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

        match input_text.trim().parse::<u32>() {
            Ok(i) => {
                let x = fib.into_iter().skip(i as usize).next().unwrap();
                println!("{} : {} ({})", i, x.to_string(), x.get_code());
            }
            Err(..) => break,
        };
    }
}


struct Fib {
    is_zero: bool,
    z: (BigInteger, BigInteger),
}

impl Fib {
    pub fn new() -> Fib {
        Fib {
            is_zero: true,
            z: (BigInteger::one(), BigInteger::zero()),
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
            self.z = (self.z.1.clone(), (&self.z.0 + &self.z.1));
            println!("next : {}", self.z.1.to_string());
            Some(self.z.1.clone())
        }
    }
}
