#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

/// 更相减损术
pub fn subtraction(mut x: u64, mut y: u64) -> u64 {
    while x != y {
        if x > y {
            x -= y
        } else {
            y -= x
        }
    }
    x
}

/// 更相减损术 除2
pub fn subtraction2(mut x: u64, mut y: u64) -> u64 {
    let mut i = 1;
    while x % 2 == 0 && y % 2 == 0 {
        x = x / 2;
        y = y / 2;
        i = i * 2;
    }

    while x != y {
        if x > y {
            x -= y;
        } else if y > x {
            y -= x;
        }
    }
    x * i
}
