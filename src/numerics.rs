use std::str;
use std::ops::Add;

pub struct BigInteger {
    bits: Vec<u32>,
    sign: i32,
}

impl Clone for BigInteger {
    fn clone(&self) -> BigInteger {
        BigInteger {
            bits: self.bits.clone(),
            sign: self.sign,
        }
    }
}

impl BigInteger {
    pub fn new(source: &Vec<u32>) -> BigInteger {
        let vec = source.clone();
        BigInteger { bits: vec, sign: 1 }
    }

    pub fn from_i32(source: i32) -> BigInteger {
        let vec = vec![source as u32];
        BigInteger {
            bits: vec,
            sign: if source < 0 { -1 } else { 1 },
        }
    }

    pub fn zero() -> BigInteger {
        BigInteger {
            bits: vec![0],
            sign: 1,
        }
    }
    pub fn one() -> BigInteger {
        BigInteger {
            bits: vec![1],
            sign: 1,
        }
    }

    pub fn get_code(&self) -> String {
        let mut v = self.bits.clone();
        v.reverse();

        v.into_iter()
            .map(|x| format!("{:08x}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
impl<'a, 'b> Add<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn add(self, other: &'b BigInteger) -> BigInteger {
        let mut buf = Vec::<u32>::new();
        let mut carry = 0u32;
        let mut index = 0;

        loop {
            if index >= self.bits.len() && index >= other.bits.len() {
                break;
            }

            let mut num = carry as u64;

            if index < self.bits.len() {
                num += self.bits[index] as u64;
            }
            if index < other.bits.len() {
                num += other.bits[index] as u64;
            }

            buf.push(num as u32);

            carry = (num >> 32) as u32;

            index += 1;
        }

        if carry > 0 {
            buf.push(carry);
        }

        BigInteger { bits: buf, sign: 1 }
    }
}

impl ToString for BigInteger {
    fn to_string(&self) -> String {


        let _bits = &self.bits;
        let _sign = self.sign;
        let negative_sign = "-";

        if _bits.len() == 0 {
            return "0".to_string();
        }


        let block_bit_length = 32;

        // 2^32 = 4.3e9
        // First convert to base 10^9.
        let ku_base = 1000_000_000u32; // 10^9
        let ku_base64 = ku_base as u64;
        let kcch_base = 9;

        let cu_src = _bits.len();
        let cu_max = cu_src * 10 / 9 + 2;
        let mut rgu_dst = vec![0u32; cu_max];
        let mut cu_dst = 0;

        for iu_src in (0..cu_src).map(|x| cu_src - 1 - x) {
            let mut u_carry = _bits[iu_src];


            for iu_dst in 0..cu_dst {
                let uu_res = ((rgu_dst[iu_dst] as u64) << block_bit_length) | u_carry as u64;
                rgu_dst[iu_dst] = (uu_res % ku_base64) as u32;
                u_carry = (uu_res / ku_base64) as u32;
            }
            if u_carry != 0 {
                rgu_dst[cu_dst] = u_carry % ku_base;
                cu_dst += 1;
                u_carry /= ku_base;
                if u_carry != 0 {
                    rgu_dst[cu_dst] = u_carry;
                    cu_dst += 1;
                }
            }
        }

        if cu_dst == 0 {
            return "0".to_string();
        }

        //numberTextLength
        // Each uint contributes at most 9 digits to the decimal representation.
        let mut cch_max = cu_dst * kcch_base;

        if _sign < 0 {
            // Leave an extra slot for a minus sign.
            cch_max += negative_sign.chars().count();
        }


        // We'll pass the rgch buffer to native code, which is going to treat it like a string of digits,
        // so it needs to be null terminated.
        // Let's ensure that we can allocate a buffer of that size.
        let rgch_buf_size = cch_max + 1;


        let mut rgch = vec![0u8; rgch_buf_size];

        let mut ich_dst = cch_max;

        for item in rgu_dst.iter().take(cu_dst as usize) {
            let mut u_dig = *item;
            for _ in 0..kcch_base {
                ich_dst -= 1;
                rgch[ich_dst] = '0' as u8 + (u_dig % 10) as u8;
                u_dig /= 10;
            }
        }

        while ich_dst < cch_max && rgch[ich_dst] == '0' as u8 {
            ich_dst += 1;
        }

        if _sign < 0 {
            let text = negative_sign.chars().collect::<Vec<char>>();
            let len = text.len();
            for i in (0..len).map(|x| len - 1 - x) {
                ich_dst -= 1;
                rgch[ich_dst] = text[i] as u8;
            }
        }

        rgch[ich_dst..cch_max]
            .iter()
            .map(|&x| x as char)
            .collect()
    }
}
