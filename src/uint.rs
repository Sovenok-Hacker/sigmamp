use std::{
    cmp::Ordering,
    ops::{AddAssign, SubAssign},
};

#[cfg(target_pointer_width = "64")]
pub type Limb = u64;

#[cfg(target_pointer_width = "32")]
pub type Limb = u32;

/// Arbirary large unsigned integer
#[derive(Debug, Eq, Clone)]
pub struct SigmaUInt {
    // Number polynomial coefficients representation
    // Starts from the least significant limb
    // If it has trailing zeros the universe will likely collapse :)
    limbs: Vec<Limb>,
}

impl SigmaUInt {
    pub fn zero() -> Self {
        SigmaUInt { limbs: vec![] }
    }
    pub fn from_usize(n: usize) -> Self {
        if n == 0 {
            return Self::zero();
        }
        SigmaUInt {
            limbs: vec![n as Limb],
        }
    }
    pub fn from_limbs(mut limbs: Vec<Limb>) -> Self {
        // Under no circumstances don't create uints from limb arrays that have trailing zeros

        while limbs.last() == Some(&0) {
            limbs.pop();
        }
        Self { limbs }
    }
    pub fn to_limbs(self: Self) -> Vec<Limb> {
        self.limbs
    }
}

impl AddAssign<&SigmaUInt> for SigmaUInt {
    fn add_assign(&mut self, rhs: &Self) {
        let mut carry: Limb = 0;
        let mut c: bool;

        // Extend self if rhs is longer
        if self.limbs.len() < rhs.limbs.len() {
            self.limbs.resize(rhs.limbs.len(), 0);
        }

        // Addition itself, with carry
        for (i, l) in rhs.limbs.iter().enumerate() {
            (self.limbs[i], c) = self.limbs[i].overflowing_add(carry);
            carry = 0;
            if c {
                carry += 1;
            }
            (self.limbs[i], c) = self.limbs[i].overflowing_add(*l);
            if c {
                carry += 1;
            }
        }

        // Propagate the carry
        let mut i = rhs.limbs.len();
        while carry > 0 && i < self.limbs.len() {
            (self.limbs[i], c) = self.limbs[i].overflowing_add(carry);
            carry = 0;
            if c {
                carry = 1;
            }
            i += 1;
        }

        // If that damn carry is still not zero, we add it in the end
        if carry != 0 {
            self.limbs.push(carry);
        }
    }
}

impl SubAssign<&SigmaUInt> for SigmaUInt {
    fn sub_assign(&mut self, rhs: &SigmaUInt) {
        let mut carry: Limb = 0;
        let mut c: bool;

        // If minuend is smaller than subtrahend, the result must be negative, therefore the subtraction is impossible in unsigned integers
        if self.limbs.len() < rhs.limbs.len() {
            panic!("Overflow, subtraction resulted in negative number")
        }

        // Subtraction itself, with carry
        for (i, l) in rhs.limbs.iter().enumerate() {
            (self.limbs[i], c) = self.limbs[i].overflowing_sub(carry);
            carry = 0;
            if c {
                carry += 1;
            }
            (self.limbs[i], c) = self.limbs[i].overflowing_sub(*l);
            if c {
                carry += 1;
            }
        }

        // Propagate the carry
        let mut i = rhs.limbs.len();
        while carry > 0 && i < self.limbs.len() {
            (self.limbs[i], c) = self.limbs[i].overflowing_sub(carry);
            carry = 0;
            if c {
                carry = 1;
            }
            i += 1;
        }

        // If that damn carry is still not zero, the result must be negative, therefore the subtraction is impossible in unsigned integers
        if carry > 0 {
            panic!("Overflow, subtraction resulted in negative number")
        }

        // I said no trailing zeros!
        while self.limbs.len() > 0 && self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }
}

// impl Mul<&SigmaUInt> for &SigmaUInt {
//     fn mul(self, rhs: &SigmaUInt) -> Self::Output {}
// }

impl PartialEq for SigmaUInt {
    fn eq(&self, other: &Self) -> bool {
        if self.limbs.len() != other.limbs.len() {
            return false;
        }

        for (a, b) in self.limbs.iter().zip(other.limbs.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for SigmaUInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.limbs.len() != other.limbs.len() {
            return self.limbs.len().partial_cmp(&other.limbs.len());
        }

        for (a, b) in self.limbs.iter().zip(other.limbs.iter()).rev() {
            if a != b {
                return a.partial_cmp(b);
            }
        }

        Some(Ordering::Equal)
    }
}
