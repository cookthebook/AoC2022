use util::readln;
use hal::serial::{FullConfig, Serial};
use core::fmt::Write;

use crate::util::strtoul;

pub fn solve(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];
    let mut score: u32 = 0;

    writeln!(uart, "Read challenge until empty line").ok();

    loop {
        let cnt = readln(uart, &mut buf);

        if cnt != 3 {
            break;
        }

        /*
         * A, X = rock (1 pt)
         * B, Y = paper (2 pt)
         * C, Z = scissors (3 pt)
         */

        /* match on your guess */
        match buf[2] {
        b'X' => {
            score += 1;
            if buf[0] == b'A' {
                score += 3;
            } else if buf[0] == b'C' {
                score += 6;
            }
        },

        b'Y' => {
            score += 2;
            if buf[0] == b'B' {
                score += 3;
            } else if buf[0] == b'A' {
                score += 6;
            }
        },

        b'Z' => {
            score += 3;
            if buf[0] == b'C' {
                score += 3;
            } else if buf[0] == b'B' {
                score += 6;
            }
        },

        _ => {
            writeln!(uart, "Invalid input").ok();
        }
        }
    }

    writeln!(uart, "Total score: {}", score).ok();
}

pub fn solve_star(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];
    let mut score: u32 = 0;

    writeln!(uart, "Read challenge until empty line").ok();

    loop {
        let cnt = readln(uart, &mut buf);

        if cnt != 3 {
            break;
        }

        /*
         * A = rock (1 pt)
         * B = paper (2 pt)
         * C = scissors (3 pt)
         * X = lose (0 pt)
         * Y = draw (3 pt)
         * Z = win (6 pt)
         */

        /* match on your guess */
        match buf[2] {
        b'X' => {
            match buf[0] {
            b'A' => { score += 3; },
            b'B' => { score += 1; },
            b'C' => { score += 2; },
            _ => {}
            }
        },

        b'Y' => {
            score += 3;

            match buf[0] {
            b'A' => { score += 1; },
            b'B' => { score += 2; },
            b'C' => { score += 3; },
            _ => {}
            }
        },

        b'Z' => {
            score += 6;

            match buf[0] {
            b'A' => { score += 2; },
            b'B' => { score += 3; },
            b'C' => { score += 1; },
            _ => {}
            }
        },

        _ => {
            writeln!(uart, "Invalid input").ok();
        }
        }
    }

    writeln!(uart, "Total score: {}", score).ok();
}
