extern crate bit_vec;
use bit_vec::BitVec;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::PartialEq;
use std::cmp::Ordering;
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
    fn new(val: i64, size: usize) -> ivec {
        let mut new = ivec{bv: BitVec::from_elem(size as usize,false)};
        let neg = val < 0;
        let val = val.abs();
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
        let neg = self.bv.get(255).expect(msg);
        let mut pow = 0u32;
        let mut ret = 0i64;
        let two = 2u32;
        while pow < 64 {
            if !neg && self.bv.get(pow as usize) == Some(true){
                ret = ret + two.pow(pow) as i64;
            }
            else if neg && self.bv.get(pow as usize) == Some(true){
                ret = ret - two.pow(pow) as i64;
            }
            pow = pow + 1;
        }

        ret
    }
}

impl ivec {
    fn twos_comp(&self) -> ivec {
        let one = ivec::new(1, self.bv.len());
        let mut ret = ivec::copy(self);
        ret.bv.negate();
        ret + one
    }
}


impl ivec {
    fn copy(&self) -> ivec {
        let msg = "Can't copy vec with unset bits!";
        let mut count = (self.bv.len() - 1) as i32;
        let mut ret = ivec::new(0, self.bv.len());
        while count >= 0 {
            ret.bv.set(count as usize, self.bv.get(count as usize).expect(msg));
            count = count - 1;
        }
        ret
    }
}

impl ivec {
    fn is_neg(&self) -> bool {
        let msg = "A bitvector with no bits is neither positive or negative!";
        self.bv.get(self.bv.len() - 1).expect(msg)
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
        else if !self.is_neg() && !other.is_neg() {
            let msg = "Can't compare numbers with unset bits!";
            let mut count = (self.bv.len() - 1) as i32;
            while count >= 0 {
                if self.bv.get(count as usize) == Some(true) &&
                   other.bv.get(count as usize) == Some(false) {
                   return Ordering::Greater
                }
                if self.bv.get(count as usize) == Some(false) &&
                   other.bv.get(count as usize) == Some(true) {
                   return Ordering::Less
                }
                count = count - 1;
            }
            Ordering:: Equal
        }
        else {
            self.twos_comp().cmp(&other.twos_comp())
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

impl Eq for ivec {}

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
        let d = ivec::new(0, 256);
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
        let d = ivec::new(4294966, 256);
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
        let a = ivec::new(4294966, 256);
        let b = ivec::new(2147483, 256);
        let c = a - b;
        let d = ivec::new(2147483, 256);
        assert_eq!(c.bv, d.bv);
    }

    #[test]
    fn  eq() {
        let a = ivec::new(2147483, 256);
        let b = ivec::new(2147483, 256);
        assert_eq!(a,b);
        assert_eq!(a.bv, b.bv);
    }

    #[test]
    fn  gv() {
        let a = ivec::new(2147483, 256);
        let b = ivec::get_val(a);
        assert_eq!(2147483, b);
    }

    #[test]
    fn  cmp() {
        let a = ivec::new(2147483, 256);
        let b = ivec::new(2147484, 256);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(b > a, true);
        assert_eq!(a < b, true);
        assert_eq!(a > b, false);
        assert_eq!(b < a, false);
    }
}
