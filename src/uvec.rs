use self::BitVec;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::PartialEq;
use std::cmp::Ordering
use std::fmt;

struct uvec {
    bv: BitVec
}

impl fmt::Debug for uvec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uvec: {:?}", self.bv)
    }
}

impl uvec {
    fn new(val: u64, size: u64) -> uvec {
        let mut new = uvec{bv: BitVec::from_elem(size,false)};
        let mut to_add = val as u64;
        let mut pow = 62i8;
        let two = 2u64;
        while pow >= 0 {
            if two.pow(pow as u32) <= to_add {
                to_add = to_add - two.pow(pow as u32);
                new.bv.set(pow as usize, true);
            }
            pow = pow - 1;
        }
        new
    }
}

impl uvec {
    fn get_val(self) -> i64 {
        let pow = 0i8;
        let ret = 0i64;
        let two = 2u8;
        while pow < 64 {
            if self.get(pow) == Some(true){
                ret = ret + two.pow(pow);
            }
            pow = pow + 1;
        }

        ret
    }
}

impl uvec {
    fn twos_comp(mut self) -> uvec {
        let one = uvec::new(1);
        self.bv.negate();
        self + one
    }
}

impl uvec {
    fn is_neg(self) -> bool {
        let msg = "A bitvector with no bits is neither positive or negative!"
        self.get(self.len).expect(msg)
    }
}

impl Add for uvec {
    type Output = uvec;

    fn add(mut self, other: uvec) -> uvec {
         assert_eq!(self.bv.len(), other.bv.len());
         // Look at the sub operator, since we only ever flip the other val
         // it should be impossible to have a negative self value
         assert_eq!(self.is_neg(), false);
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
         // Underflow!
         assert_eq!(self.is_neg(), false);
         self
    }
}

impl Sub for uvec {
    type Output = uvec;
    fn sub(self, other: uvec) -> uvec {
        self + other.twos_comp()
    }
}

impl Ord for uvec {
    fn cmp(&self, other: &uvec) -> Ordering {
        assert_eq!(self.is_neg(), false);
        assert_eq!(other.is_neg(), false);
        if self == other {
            Ordering::Equal
        }
        else {
            let count = (self.len - 1) as i32;
            while count >= 0 {

                count = count - 1;
            }
        }
    }
}

impl PartialOrd for uvec {
    fn partial_cmp(&self, other: &uvec) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for uvec {
    fn eq(&self, other: &uvec) -> bool {
        self.bv == other.bv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let test = uvec::new(1, 256);
        let mut hardcoded = BitVec::from_elem(256, false);
        hardcoded.set(0, true);
        assert_eq!(test.bv, hardcoded);
    }

    #[test]
    fn neg_one() {
        let test = uvec::new(-1, 256);
        let hardcoded = BitVec::from_elem(256, true);
        assert_eq!(test.bv, hardcoded);
    }

    #[test]
    fn big_pos() {
        let test = uvec::new(2147483, 256);
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
        let test = uvec::new(-2147483, 256);
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
        let a = uvec::new(-2147483, 256);
        let b = uvec::new(2147483, 256);
        let c = a + b;
        let d = uvec::new(0);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  negative_add() {
        let a = uvec::new(-2147483, 256);
        let b = uvec::new(-2147483, 256);
        let c = a + b;
        let d = uvec::new(-4294966, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  pos_add() {
        let a = uvec::new(2147483, 256);
        let b = uvec::new(2147483, 256);
        let c = a + b;
        let d = uvec::new(4294966);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  negative_sub() {
        let a = uvec::new(-2147483, 256);
        let b = uvec::new(-2147483, 256);
        let c = a - b;
        let d = uvec::new(0, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  pos_sub() {
        let a = uvec::new(2147483, 256);
        let b = uvec::new(2147483, 256);
        let c = a + b;
        let d = uvec::new(0, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  eq() {
        let a = uvec::new(2147483, 256);
        let b = uvec::new(2147483, 256);
        assert_eq!(a,b);
        assert_eq!(a.bv, b.bv);
    }
}
