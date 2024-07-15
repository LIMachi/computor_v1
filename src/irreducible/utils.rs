use std::ops::{Neg, Rem};

pub fn gcd<K: Default + Copy + PartialEq + Neg<Output = K> + PartialOrd + Rem<Output = K>>(mut v1: K, mut v2: K) -> K {
    let zero = K::default();
    if v1 == zero || v2 == zero {
        zero //technically an error
    } else {
        if v1 < zero {
            v1 = -v1;
        }
        if v2 < zero {
            v2 = -v2;
        }
        if v1 > v2 {
            let t = v1;
            v1 = v2;
            v2 = t;
        }
        while v1 != zero {
            let t = v1;
            v1 = v2 % v1;
            v2 = t;
        }
        v2
    }
}