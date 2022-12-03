use util::readln;
use hal::serial::{FullConfig, Serial};
use core::fmt::Write;

fn byte_to_prio(b: u8) -> u8 {
    if b >= b'a' && b <= b'z' {
        return b - b'a' + 1;
    } else if b >= b'A' && b <= b'Z' {
        return b - b'A' + 27;
    }

    return 0;
}

pub fn solve(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut result: u32 = 0;
    let mut buf = [0u8; 128];

    write!(uart, "=== Day 03, normal ===\r\n\r\n").ok();
    write!(uart, "Input challenge lines until empty line\r\n").ok();

    loop {
        /* The bag can only have 52 types */
        let mut bag_mask: u64 = 0;
        let rlen = readln(uart, &mut buf);

        if rlen % 2 != 0 {
            write!(uart, "Invalid input (odd item count)\r\n").ok();
            return;
        }
        if rlen == 0 {
            break;
        }

        for i in 0..(rlen/2) {
            let prio = byte_to_prio(buf[i]);
            if prio == 0 {
                write!(uart, "Invalid bag item {}\r\n", buf[i]).ok();
                return;
            }

            bag_mask |= 1 << (prio - 1);
        }

        // write!(uart, "First half bag mask: {}\r\n", bag_mask).ok();

        for i in (rlen/2)..rlen {
            let prio = byte_to_prio(buf[i]);
            if prio == 0 {
                write!(uart, "Invalid bag item {}\r\n", buf[i]).ok();
                return;
            }

            if (bag_mask & (1 << (prio - 1))) != 0 {
                result += prio as u32;
                break;
            }
        }
    }

    write!(uart, "Total result: {}\r\n", result).ok();
}

pub fn solve_star(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];
    let mut score: u32 = 0;

    write!(uart, "=== Day 03, star ===\r\n\r\n").ok();
    write!(uart, "Input challenge lines until empty line\r\n").ok();
}
