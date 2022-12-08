use util;
use core::fmt::Write;
use xmodem;

use crate::util::strtoul;

pub fn solve(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut max: u128 = 0;
    let mut cur: u128 = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret: u8;

    write!(p.uart, "=== Day 01, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
                b'\n' => {
                    line[linep] = 0;

                    /* crunch this result */
                    if linep == 0 {
                        /* empty line, check if this is our new max */
                        if cur > max {
                            max = cur;
                        }

                        cur = 0;
                    } else {
                        cur += strtoul(&line, 10) as u128;
                    }

                    linep = 0;
                },

                xmodem::XMODEM_PAD => { /* ignore pad characters */ },

                _ => {
                    line[linep] = buf[i];
                    linep += 1;

                    /* we got a line bigger than a single XModem packet */
                    if linep == (xmodem::BLOCK_SZ - 1) {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Invalid input\r\n").ok();
                        return;
                    }
                }
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    writeln!(p.uart, "Max calories: {}", max).ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut max1: u128 = 0;
    let mut max2: u128 = 0;
    let mut max3: u128 = 0;
    let mut cur: u128 = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret: u8;

    write!(p.uart, "=== Day 01, star ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
                b'\n' => {
                    line[linep] = 0;

                    /* crunch this result */
                    if linep == 0 {
                        /* empty line, check if this is our new max */
                        if cur > max1 {
                            max3 = max2;
                            max2 = max1;
                            max1 = cur;
                        }
                        else if cur > max2 {
                            max3 = max2;
                            max2 = cur;
                        }
                        else if cur > max3 {
                            max3 = cur;
                        }

                        cur = 0;
                    } else {
                        cur += strtoul(&line, 10) as u128;
                    }

                    linep = 0;
                },

                xmodem::XMODEM_PAD => { /* ignore pad characters */ },

                _ => {
                    line[linep] = buf[i];
                    linep += 1;

                    /* we got a line bigger than a single XModem packet */
                    if linep == (xmodem::BLOCK_SZ - 1) {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Invalid input\r\n").ok();
                        return;
                    }
                }
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    writeln!(p.uart, "Max calories: {}, {}, {}", max1, max2, max3).ok();
    writeln!(p.uart, "Total: {}", max1+max2+max3).ok();
}
