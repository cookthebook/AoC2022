use util::readln;
use rtt_target::{UpChannel, DownChannel};
use core::fmt::Write;

use crate::util::strtoul;

pub fn solve(input: &mut DownChannel, output: &mut UpChannel) {
    let mut buf = [0u8; 16];
    let mut max: u128 = 0;
    let mut cur: u128 = 0;
    let mut pval = 0;

    writeln!(output, "Read calory values until double newline").ok();

    loop {
        readln(input, &mut buf);
        let val = strtoul(&buf, 10);

        if val == 0 {
            if pval == 0 {
                break;
            }

            if cur > max {
                max = cur;
            }

            cur = 0;
        } else {
            cur += val as u128;
        }

        pval = val;
    }

    writeln!(output, "Max calories: {}", max).ok();
}

pub fn solve_star(input: &mut DownChannel, output: &mut UpChannel) {
    let mut buf = [0u8; 16];
    let mut max1: u128 = 0;
    let mut max2: u128 = 0;
    let mut max3: u128 = 0;
    let mut cur: u128 = 0;
    let mut pval = 0;

    writeln!(output, "Read calory values until double newline").ok();

    loop {
        readln(input, &mut buf);
        let val = strtoul(&buf, 10);

        if val == 0 {
            if pval == 0 {
                break;
            }

            if cur > max1 {
                max3 = max2;
                max2 = max1;
                max1 = cur;
            } else if cur > max2 {
                max3 = max2;
                max2 = cur;
            } else if cur > max3 {
                max3 = cur;
            }

            cur = 0;
        } else {
            cur += val as u128;
        }

        pval = val;
    }

    writeln!(output, "Max calories: {}, {}, {}", max1, max2, max3).ok();
    writeln!(output, "Total: {}", max1+max2+max3).ok();
}
