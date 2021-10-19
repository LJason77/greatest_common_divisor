#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::must_use_candidate)]

use std::cmp::Ordering;

/// 更相减损术
pub fn subtraction(mut x: u128, mut y: u128) -> u128 {
    loop {
        match x.cmp(&y) {
            Ordering::Less => y -= x,
            Ordering::Greater => x -= y,
            Ordering::Equal => return x,
        }
    }
}

/// 更相减损术 除2
pub fn subtraction2(mut x: u128, mut y: u128) -> u128 {
    let mut i = 1;
    while x & 1 == 0 && y & 1 == 0 {
        x >>= 1;
        y >>= 1;
        i <<= 1;
    }

    loop {
        match x.cmp(&y) {
            Ordering::Less => y -= x,
            Ordering::Greater => x -= y,
            Ordering::Equal => return x * i,
        }
    }
}

/// 辗转相除法
pub fn euclidean(mut x: u128, mut y: u128) -> u128 {
    while x * y != 0 {
        match x.cmp(&y) {
            Ordering::Less => {
                y %= x;
            }
            Ordering::Greater => {
                x %= y;
            }
            Ordering::Equal => return y,
        }
    }
    match x.cmp(&y) {
        Ordering::Less | Ordering::Equal => y,
        Ordering::Greater => x,
    }
}
