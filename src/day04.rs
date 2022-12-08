use core::fmt::Write;
use xmodem;
use util;

pub fn solve(p: &mut util::CorePerphs) {
    let mut result: u32 = 0;
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    write!(p.uart, "=== Day 04, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
            b'\n' => {
                let mut nump = 0;
                let mut num_len;
                let mut r1 = [0u64; 2];
                let mut r2 = [0u64; 2];

                line[linep] = 0;

                /* expect format <num>-<num>,<num>-<num> */
                (r1[0], num_len) = util::strtoul_len(&line[nump..], 10);
                nump += num_len + 1;
                (r1[1], num_len) = util::strtoul_len(&line[nump..], 10);
                nump += num_len + 1;
                (r2[0], num_len) = util::strtoul_len(&line[nump..], 10);
                nump += num_len + 1;
                r2[1] = util::strtoul(&line[nump..], 10);

                /* one range fully contains the other */
                if ((r1[0] <= r2[0]) && (r1[1] >= r2[1])) ||
                   ((r2[0] <= r1[0]) && (r2[1] >= r1[1])) {
                    result += 1;
                }

                linep = 0;
            },

            xmodem::XMODEM_PAD => { /* ignore pad characters */ },

            _ => {
                line[linep] = buf[i];
                linep += 1;

                if linep == (xmodem::BLOCK_SZ - 1) {
                    xm.xmodem_cancel(p);
                    write!(p.uart, "Line too large\r\n").ok();
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
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    write!(p.uart, "=== Day 04, star ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
            b'\n' => {
                let mut nump = 0;
                let mut num_len;
                let mut r1 = [0u64; 2];
                let mut r2 = [0u64; 2];

                line[linep] = 0;

                /* expect format <num>-<num>,<num>-<num> */
                (r1[0], num_len) = util::strtoul_len(&line[nump..], 10);
                nump += num_len + 1;
                (r1[1], num_len) = util::strtoul_len(&line[nump..], 10);
                nump += num_len + 1;
                (r2[0], num_len) = util::strtoul_len(&line[nump..], 10);
                nump += num_len + 1;
                r2[1] = util::strtoul(&line[nump..], 10);

                /* ranges overlap */
                if (r1[0] >= r2[0] && r1[0] <= r2[1]) ||
                   (r2[0] >= r1[0] && r2[0] <= r1[1]) {
                    result += 1;
                }

                linep = 0;
            },

            xmodem::XMODEM_PAD => { /* ignore pad characters */ },

            _ => {
                line[linep] = buf[i];
                linep += 1;

                if linep == (xmodem::BLOCK_SZ - 1) {
                    xm.xmodem_cancel(p);
                    write!(p.uart, "Line too large\r\n").ok();
                    return;
                }
            }
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    write!(p.uart, "Total result: {}\r\n", result).ok();
}
