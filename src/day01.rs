use util::readln;
use util;
use hal::serial::{FullConfig, Serial};
use core::fmt::Write;
use xmodem;

use crate::util::strtoul;

pub fn solve(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut bufp = 0;
    let mut max: u128 = 0;
    let mut cur: u128 = 0;
    let mut plen = 0;

    write!(p.uart, "=== Day 01, normal ===\r\n\r\n").ok();
    write!(p.uart, "Beginning XModem transfer...\r\n").ok();

    let proc_val = |val: u128| {
        if val > max {
            max = val;
        }
    };

    let xmodem_cb = |pkt: [u8; xmodem::BLOCK_SZ]| {
        for p in 0..xmodem::BLOCK_SZ {
            match pkt[p] {
                b'\r' | xmodem::XMODEM_PAD => {
                    /* ignore */
                }

                b'\n' => {
                    /* crunch this result */
                    proc_val(strtoul(&buf, 10) as u128);
                }
            }
        }
    };

    xmodem::xmodem_recv(p, xmodem_cb);

    loop {
        let rlen = readln(&mut p.uart, &mut buf);

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

    writeln!(p.uart, "Max calories: {}", max).ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut buf = [0u8; 16];
    let mut max1: u128 = 0;
    let mut max2: u128 = 0;
    let mut max3: u128 = 0;
    let mut cur: u128 = 0;
    let mut plen = 0;

    write!(p.uart, "=== Day 01, star ===\r\n\r\n").ok();
    write!(p.uart, "Gimme da calory values until double newline\r\n").ok();

    loop {
        let rlen = readln(&mut p.uart, &mut buf);

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

    writeln!(p.uart, "Max calories: {}, {}, {}", max1, max2, max3).ok();
    writeln!(p.uart, "Total: {}", max1+max2+max3).ok();
}
