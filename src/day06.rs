use core::fmt::Write;
use xmodem;
use util;

fn all_different(buf: &[u8]) -> bool {
    let mut mask: u32 = 0;

    for i in 0..buf.len() {
        if !buf[i].is_ascii_lowercase() {
            return false;
        }

        let bit = 1 << (buf[i] - b'a');
        if mask & bit != 0 {
            return false;
        }

        mask |= bit;
    }

    return true;
}

pub fn solve(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut check = [0u8; 4];
    let mut checkp = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;
    let mut result = 0;
    let mut found = false;

    write!(p.uart, "=== Day 06, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */

        for i in 0..xmodem::BLOCK_SZ {
            if !found && buf[i].is_ascii_lowercase() {
                check[checkp] = buf[i];
                checkp += 1;
                result += 1;

                if result >= 4 && all_different(&check) {
                    found = true;
                }

                if checkp == 4 {
                    checkp = 0;
                }
            } else {
                /* ingore when found or non-ascii */
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    write!(p.uart, "Result: {}\r\n", result).ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut check = [0u8; 14];
    let mut checkp = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;
    let mut result = 0;
    let mut found = false;

    write!(p.uart, "=== Day 06, star ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */

        for i in 0..xmodem::BLOCK_SZ {
            if !found && buf[i].is_ascii_lowercase() {
                check[checkp] = buf[i];
                checkp += 1;
                result += 1;

                if result >= 14 && all_different(&check) {
                    found = true;
                }

                if checkp == 14 {
                    checkp = 0;
                }
            } else {
                /* ingore when found or non-ascii */
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    write!(p.uart, "Result: {}\r\n", result).ok();
}
