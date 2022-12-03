use util::readln;
use hal::serial::{FullConfig, Serial};
use core::fmt::Write;

use crate::util::strtoul;

pub fn solve(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];
    let mut max: u128 = 0;
    let mut cur: u128 = 0;
    let mut plen = 0;

    write!(uart, "=== Day 01, normal ===\r\n\r\n").ok();
    write!(uart, "Gimme da calory values until double newline\r\n").ok();

    loop {
        let rlen = readln(uart, &mut buf);

        if rlen == 0 {
            if plen == 0 {
                break;
            }

            if cur > max {
                max = cur;
            }

            cur = 0;
        } else {
            cur += strtoul(&buf, 10) as u128;
        }

        plen = rlen;
    }

    writeln!(uart, "Max calories: {}", max).ok();
}

pub fn solve_star(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];
    let mut max1: u128 = 0;
    let mut max2: u128 = 0;
    let mut max3: u128 = 0;
    let mut cur: u128 = 0;
    let mut plen = 0;

    write!(uart, "=== Day 01, star ===\r\n\r\n").ok();
    write!(uart, "Gimme da calory values until double newline\r\n").ok();

    loop {
        let rlen = readln(uart, &mut buf);

        if rlen == 0 {
            if plen == 0 {
                break;
            }

            if cur > max1 {
                max3 = max2;
                max2 = max1;
                max1 = cur;
            } else if cur > max2 {
                max3 = max2;
                max2 = cur
            } else if cur > max3 {
                max3 = cur;
            }

            cur = 0;
        } else {
            cur += strtoul(&buf, 10) as u128;
        }

        plen = rlen;
    }

    writeln!(uart, "Max calories: {}, {}, {}", max1, max2, max3).ok();
    writeln!(uart, "Total: {}", max1+max2+max3).ok();
}
