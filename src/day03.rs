use core::fmt::Write;
use xmodem;
use util;

fn byte_to_prio(b: u8) -> u8 {
    if b >= b'a' && b <= b'z' {
        return b - b'a' + 1;
    } else if b >= b'A' && b <= b'Z' {
        return b - b'A' + 27;
    }

    return 0;
}

fn mask_to_prio(m: u64) -> u8 {
    for i in 0..64 {
        if m & (1 << i) != 0 {
            return i+1;
        }
    }

    return 0;
}

pub fn solve(p: &mut util::CorePerphs) {
    let mut result: u32 = 0;
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    write!(p.uart, "=== Day 03, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
                b'\n' => {
                    let mut bag_mask: u64 = 0;
                    line[linep] = 0;

                    if (linep % 2) != 0 {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Invalid input (odd item count)\r\n").ok();
                        return;
                    }

                    for j in 0..(linep/2) {
                        let prio = byte_to_prio(line[j]);

                        if prio == 0 {
                            xm.xmodem_cancel(p);
                            write!(p.uart, "Invalid bag item {}\r\n", line[j]).ok();
                            return;
                        }

                        bag_mask |= 1 << (prio - 1);
                    }

                    for j in (linep/2)..linep {
                        let prio = byte_to_prio(line[j]);
                        if prio == 0 {
                            xm.xmodem_cancel(p);
                            write!(p.uart, "Invalid bag item {}\r\n", line[j]).ok();
                            return;
                        }

                        if (bag_mask & (1 << (prio - 1))) != 0 {
                            result += prio as u32;
                            break;
                        }
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

    write!(p.uart, "Total result: {}\r\n", result).ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut result: u32 = 0;
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut line_cnt = 0;
    let mut masks = [0u64; 3];
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    write!(p.uart, "=== Day 03, star ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
                b'\n' => {
                    let mut bag_mask: u64 = 0;
                    line[linep] = 0;

                    if (linep % 2) != 0 {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Invalid input (odd item count)\r\n").ok();
                        return;
                    }

                    for j in 0..linep {
                        let prio = byte_to_prio(line[j]);

                        if prio == 0 {
                            xm.xmodem_cancel(p);
                            write!(p.uart, "Invalid bag item {}\r\n", line[j]).ok();
                            return;
                        }

                        bag_mask |= 1 << (prio - 1);
                    }

                    masks[line_cnt] = bag_mask;
                    line_cnt += 1;

                    if line_cnt == 3 {
                        bag_mask = masks[0] & masks[1] & masks[2];
                        result += mask_to_prio(bag_mask) as u32;
                        line_cnt = 0;
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

    write!(p.uart, "Total result: {}\r\n", result).ok();
}
