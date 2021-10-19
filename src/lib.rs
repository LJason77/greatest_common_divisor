#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::must_use_candidate)]

use std::cmp::Ordering;

/// 更相减损术
pub fn subtraction(mut x: u64, mut y: u64) -> u64 {
    loop {
        match x.cmp(&y) {
            Ordering::Less => y -= x,
            Ordering::Greater => x -= y,
            Ordering::Equal => return x,
        }
    }
}

/// 更相减损术 除2
pub fn subtraction2(mut x: u64, mut y: u64) -> u64 {
    let mut i = 1;
    while x % 2 == 0 && y % 2 == 0 {
        x /= 2;
        y /= 2;
        i *= 2;
    }

    loop {
        match x.cmp(&y) {
            Ordering::Less => y -= x,
            Ordering::Greater => x -= y,
            Ordering::Equal => return x * i,
        }
    }
}
