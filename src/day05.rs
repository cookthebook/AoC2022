use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_serial_Write;
use xmodem;
use util;
use nb::block;

use crate::util::{strtoul_len, strtoul};

#[derive(Copy, Clone)]
struct Stack {
    boxes: [u8; 128],
    cnt: usize
}

impl Stack {
    fn reverse(&mut self) {
        for i in 0..(self.cnt/2) {
            let tmp = self.boxes[i];
            self.boxes[i] = self.boxes[self.cnt-1-i];
            self.boxes[self.cnt-1-i] = tmp;
        }
    }
}

fn parse_stack_line(line: &[u8], len: usize, stacks: &mut [Stack]) {
    /* expect like "[S] [J] [C]     [F] [C]     [D] [G]" */
    let mut p = 1;
    let mut idx = 0;

    while p < len {
        if line[p] != b' ' {
            stacks[idx].boxes[stacks[idx].cnt] = line[p];
            stacks[idx].cnt += 1;
        }

        p += 4;
        idx += 1;
    }
}

fn parse_move_line(line: &[u8]) -> (u8, usize, usize) {
    /* expect "move <num> from <num> to <num>" */
    let mut p = 5;
    let mut len;
    let cnt;
    let src;
    let dst;

    (cnt, len) = strtoul_len(&line[p..], 10);
    p += len + 6;
    (src, len) = strtoul_len(&line[p..], 10);
    p += len + 4;
    dst = strtoul(&line[p..], 10);

    return (cnt as u8, src as usize, dst as usize);
}

fn move_boxes(stacks: &mut [Stack], src: usize, dst: usize, cnt: u8) {
    for _ in 0..cnt {
        stacks[dst].boxes[stacks[dst].cnt] = stacks[src].boxes[stacks[src].cnt-1];
        stacks[dst].cnt += 1;
        stacks[src].cnt -= 1;
    }
}

fn move_boxes_ordered(stacks: &mut [Stack], src: usize, dst: usize, cnt: u8) {
    for i in 0..cnt {
        stacks[dst].boxes[stacks[dst].cnt + i as usize] =
            stacks[src].boxes[stacks[src].cnt - cnt as usize + i as usize];
    }
    stacks[dst].cnt += cnt as usize;
    stacks[src].cnt -= cnt as usize;
}

pub fn solve(p: &mut util::CorePerphs) {
    let mut stacks = [ Stack { boxes: [0u8; 128], cnt: 0 }; 10 ];
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;
    let mut parsing_crates = true;

    write!(p.uart, "=== Day 05, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
            b'\n' => {
                line[linep] = 0;

                if parsing_crates {
                    if line[0] == b' ' && line[1] == b'1' {
                        for i in 0..stacks.len() {
                            stacks[i].reverse();
                        }
                        parsing_crates = false;
                    } else {
                        parse_stack_line(&line, linep, &mut stacks);
                    }
                } else if linep > 0 && line[0] == b'm' {
                    let (cnt, src, dst) = parse_move_line(&line);
                    move_boxes(&mut stacks, src-1, dst-1, cnt);
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

    write!(p.uart, "Stacks: \r\n").ok();
    for i in 0..stacks.len() {
        write!(p.uart, "  {}: ", i+1).ok();
        if stacks[i].cnt == 0 {
            write!(p.uart, "empty").ok();
        }

        for j in 0..stacks[i].cnt {
            write!(p.uart, "[").ok();
            p.uart.write(stacks[i].boxes[j]).ok();
            write!(p.uart, "] ").ok();
        }

        write!(p.uart, "\r\n").ok();
    }

    write!(p.uart, "Answer: ").ok();
    for i in 0..stacks.len() {
        if stacks[i].cnt > 0 {
            block!(p.uart.write(stacks[i].boxes[stacks[i].cnt-1])).ok();
        }
    }
    write!(p.uart, "\r\n").ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut stacks = [ Stack { boxes: [0u8; 128], cnt: 0 }; 10 ];
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;
    let mut parsing_crates = true;

    write!(p.uart, "=== Day 05, star ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */
        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
            b'\n' => {
                line[linep] = 0;

                if parsing_crates {
                    if line[0] == b' ' && line[1] == b'1' {
                        for i in 0..stacks.len() {
                            stacks[i].reverse();
                        }
                        parsing_crates = false;
                    } else {
                        parse_stack_line(&line, linep, &mut stacks);
                    }
                } else if linep > 0 && line[0] == b'm' {
                    let (cnt, src, dst) = parse_move_line(&line);
                    move_boxes_ordered(&mut stacks, src-1, dst-1, cnt);
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

    write!(p.uart, "Stacks: \r\n").ok();
    for i in 0..stacks.len() {
        write!(p.uart, "  {}: ", i+1).ok();
        if stacks[i].cnt == 0 {
            write!(p.uart, "empty").ok();
        }

        for j in 0..stacks[i].cnt {
            write!(p.uart, "[").ok();
            p.uart.write(stacks[i].boxes[j]).ok();
            write!(p.uart, "] ").ok();
        }

        write!(p.uart, "\r\n").ok();
    }

    write!(p.uart, "Answer: ").ok();
    for i in 0..stacks.len() {
        if stacks[i].cnt > 0 {
            block!(p.uart.write(stacks[i].boxes[stacks[i].cnt-1])).ok();
        }
    }
    write!(p.uart, "\r\n").ok();
}
