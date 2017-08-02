use self::BitVec;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::PartialEq;
use std::cmp::Ordering
use std::fmt;

struct ivec {
    bv: BitVec
}

impl fmt::Debug for ivec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ivec: {:?}", self.bv)
    }
}

impl ivec {
    fn new(other: i64, size: u64) -> ivec {
        let mut new = ivec{bv: BitVec::from_elem(size,false)};
        let neg = other < 0;
        let other = other.abs();
        let mut to_add = other as u64;
        let mut pow = 62i8;
        let two = 2u64;
        while pow >= 0 {
            if two.pow(pow as u32) <= to_add {
                to_add = to_add - two.pow(pow as u32);
                new.bv.set(pow as usize, true);
            }
            pow = pow - 1;
        }
        if neg {
            new.twos_comp()
        }
        else {
            new
        }
    }
}

impl ivec {
    fn get_val(self) -> i64 {
        let msg = "No sign bit set for get_val";
        let neg = self.get(255).expect(msg);
        let pow = 0i8;
        let ret = 0i64;
        let two = 2u8;
        while pow < 64 {
            if !neg && self.get(pow) == Some(true){
                ret = ret + two.pow(pow);
            }
            else if neg && self.get(pow) == Some(true){
                ret = ret - two.pow(pow)
            }
            pow = pow + 1;
        }

        ret
    }
}

impl ivec {
    fn twos_comp(mut self) -> ivec {
        let one = ivec::new(1);
        self.bv.negate();
        self + one
    }
}

impl ivec {
    fn is_neg(self) -> bool {
        let msg = "A bitvector with no bits is neither positive or negative!"
        self.get(self.len).expect(msg)
    }
}

impl Add for ivec {
    type Output = ivec;

    fn add(mut self, other: ivec) -> ivec {
         assert_eq!(self.bv.len(), other.bv.len());
         let msg = "How do you expect me to add a one bit vector?";
         let neg = self.bv.get(0).expect(msg) || other.bv.get(0).expect(msg);
         let mut bit = 0;
         let mut carry = false;
         while bit < self.bv.len() {
             if self.bv.get(bit) == Some(false)
                && other.bv.get(bit) == Some(false)
                && carry == false {
                 self.bv.set(bit, false);
                 carry = false;
             }
             else if self.bv.get(bit) == Some(false)
                && other.bv.get(bit) == Some(false)
                && carry == true {
                 self.bv.set(bit, true);
                 carry = false;
             }
             else if self.bv.get(bit) == Some(false)
                && other.bv.get(bit) == Some(true)
                && carry == false {
                 self.bv.set(bit, true);
                 carry = false;
             }
             else if self.bv.get(bit) == Some(false)
                && other.bv.get(bit) == Some(true)
                && carry == true {
                 self.bv.set(bit, false);
                 carry = true;
             }
             else if self.bv.get(bit) == Some(true)
                && other.bv.get(bit) == Some(false)
                && carry == false {
                 self.bv.set(bit, true);
                 carry = false;
             }
             else if self.bv.get(bit) == Some(true)
                && other.bv.get(bit) == Some(false)
                && carry == true {
                 self.bv.set(bit, false);
                 carry = true;
             }
             else if self.bv.get(bit) == Some(true)
                && other.bv.get(bit) == Some(true)
                && carry == false {
                 self.bv.set(bit, false);
                 carry = true;
             }
             else if self.bv.get(bit) == Some(true)
                && other.bv.get(bit) == Some(true)
                && carry == true {
                 self.bv.set(bit, true);
                 carry = true;
             }
             bit = bit + 1;
         }
         // Overflow!
         assert_eq!(carry && !neg, false);
         self
    }
}

impl Sub for ivec {
    type Output = ivec;
    fn sub(self, other: ivec) -> ivec {
        self + other.twos_comp()
    }
}

impl Ord for ivec {
    fn cmp(&self, other: &ivec) -> Ordering {
        if self == other {
            Ordering::Equal
        }
        else if self.is_neg() && !other.is_neg() {
            Ordering::Less
        }
        else if !self.is_neg() && other.is_neg() {
            Ordering::Greater
        }
    }
}

impl PartialOrd for ivec {
    fn partial_cmp(&self, other: &ivec) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ivec {
    fn eq(&self, other: &ivec) -> bool {
        self.bv == other.bv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let test = ivec::new(1, 256);
        let mut hardcoded = BitVec::from_elem(256, false);
        hardcoded.set(0, true);
        assert_eq!(test.bv, hardcoded);
    }

    #[test]
    fn neg_one() {
        let test = ivec::new(-1, 256);
        let hardcoded = BitVec::from_elem(256, true);
        assert_eq!(test.bv, hardcoded);
    }

    #[test]
    fn big_pos() {
        let test = ivec::new(2147483, 256);
        let mut hardcoded = BitVec::from_elem(256, false);
        hardcoded.set(0, true);
        hardcoded.set(1, true);
        hardcoded.set(3, true);
        hardcoded.set(4, true);
        hardcoded.set(7, true);
        hardcoded.set(10, true);
        hardcoded.set(14, true);
        hardcoded.set(15, true);
        hardcoded.set(21, true);
        assert_eq!(test.bv, hardcoded);
    }

    #[test]
    fn big_neg() {
        let test = ivec::new(-2147483, 256);
        let mut hardcoded = BitVec::from_elem(256, true);
        hardcoded.set(1, false);
        hardcoded.set(3, false);
        hardcoded.set(4, false);
        hardcoded.set(7, false);
        hardcoded.set(10, false);
        hardcoded.set(14, false);
        hardcoded.set(15, false);
        hardcoded.set(21, false);
        assert_eq!(test.bv, hardcoded);
    }

    #[test]
    fn  zero_sum() {
        let a = ivec::new(-2147483, 256);
        let b = ivec::new(2147483, 256);
        let c = a + b;
        let d = ivec::new(0);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  negative_add() {
        let a = ivec::new(-2147483, 256);
        let b = ivec::new(-2147483, 256);
        let c = a + b;
        let d = ivec::new(-4294966, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  pos_add() {
        let a = ivec::new(2147483, 256);
        let b = ivec::new(2147483, 256);
        let c = a + b;
        let d = ivec::new(4294966);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  negative_sub() {
        let a = ivec::new(-2147483, 256);
        let b = ivec::new(-2147483, 256);
        let c = a - b;
        let d = ivec::new(0, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  pos_sub() {
        let a = ivec::new(2147483, 256);
        let b = ivec::new(2147483, 256);
        let c = a + b;
        let d = ivec::new(0, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  eq() {
        let a = ivec::new(2147483, 256);
        let b = ivec::new(2147483, 256);
        assert_eq!(a,b);
        assert_eq!(a.bv, b.bv);
    }
}
